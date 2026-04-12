import { App, GcsBackend, TerraformOutput, TerraformStack } from "cdktf";
import { Construct } from "constructs";
import { GoogleProvider } from "@cdktf/provider-google/lib/provider";
import { ProjectService } from "@cdktf/provider-google/lib/project-service";
import { ArtifactRegistryRepository } from "@cdktf/provider-google/lib/artifact-registry-repository";
import { ServiceAccount } from "@cdktf/provider-google/lib/service-account";
import { ProjectIamMember } from "@cdktf/provider-google/lib/project-iam-member";
import { CloudRunV2Service } from "@cdktf/provider-google/lib/cloud-run-v2-service";
import { CloudRunV2ServiceIamMember } from "@cdktf/provider-google/lib/cloud-run-v2-service-iam-member";
import { LoggingProjectBucketConfig } from "@cdktf/provider-google/lib/logging-project-bucket-config";
import { IamWorkloadIdentityPool } from "@cdktf/provider-google/lib/iam-workload-identity-pool";
import { IamWorkloadIdentityPoolProvider } from "@cdktf/provider-google/lib/iam-workload-identity-pool-provider";
import { ServiceAccountIamMember } from "@cdktf/provider-google/lib/service-account-iam-member";

const PROJECT = "talents-493111";
const REGION = "europe-west3";
const GITHUB_REPO = "kaisellgren/talents";
const IMAGE_BASE = `${REGION}-docker.pkg.dev/${PROJECT}/talents/talents`;

class TalentsStack extends TerraformStack {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    new GcsBackend(this, {
      bucket: "talents-493111-tfstate",
      prefix: "terraform/state",
    });

    new GoogleProvider(this, "google", {
      project: PROJECT,
      region: REGION,
    });

    // --- Enable required APIs ---
    const apis = [
      "run.googleapis.com",
      "artifactregistry.googleapis.com",
      "secretmanager.googleapis.com",
      "iam.googleapis.com",
      "aiplatform.googleapis.com",
      "iamcredentials.googleapis.com",
    ];
    const enabledApis: Record<string, ProjectService> = {};
    for (const api of apis) {
      enabledApis[api] = new ProjectService(this, `api-${api.replace(/\./g, "-")}`, {
        service: api,
        disableOnDestroy: false,
      });
    }

    // --- Artifact Registry (Docker) ---
    const registry = new ArtifactRegistryRepository(this, "registry", {
      repositoryId: "talents",
      format: "DOCKER",
      location: REGION,
      description: "Docker images for the talents app",
      dependsOn: [enabledApis["artifactregistry.googleapis.com"]],
    });

    // --- Cloud Run service account ---
    const runSa = new ServiceAccount(this, "run-sa", {
      accountId: "talents-run-sa",
      displayName: "Talents Cloud Run Service Account",
    });

    const runSaRoles = [
      "roles/secretmanager.secretAccessor",
      "roles/logging.logWriter",
      "roles/aiplatform.user",
    ];
    for (const role of runSaRoles) {
      new ProjectIamMember(this, `run-sa-${role.replace(/\//g, "-")}`, {
        project: PROJECT,
        role,
        member: `serviceAccount:${runSa.email}`,
      });
    }

    // --- Cloud Run service ---
    const cloudRunService = new CloudRunV2Service(this, "service", {
      name: "talents",
      location: REGION,
      ingress: "INGRESS_TRAFFIC_ALL",
      deletionProtection: false,
      template: {
        serviceAccount: runSa.email,
        scaling: {
          minInstanceCount: 0,
          maxInstanceCount: 3,
        },
        containers: [
          {
            // Placeholder used for initial bootstrap deploy.
            // CI/CD pipeline (deploy.yml) replaces this with the real image on first push to main.
            image: `us-docker.pkg.dev/cloudrun/container/hello`,
            ports: { containerPort: 3000 },
            resources: {
              limits: {
                cpu: "1",
                memory: "512Mi",
              },
              cpuIdle: true,
            },
            env: [
              { name: "RUST_LOG", value: "debug" },
              { name: "USE_GCP_AUTH", value: "true" },
              {
                name: "LLM_URL",
                // gemini-2.0-flash is only available via us-central1 on Vertex AI
                value: `https://us-central1-aiplatform.googleapis.com/v1beta1/projects/${PROJECT}/locations/us-central1/endpoints/openapi`,
              },
              { name: "LLM_MODEL", value: "google/gemini-1.5-flash-002" },
              {
                name: "DATABASE_URL",
                valueSource: {
                  secretKeyRef: {
                    secret: "database-url",
                    version: "latest",
                  },
                },
              },
            ],
          },
        ],
      },
      dependsOn: [
        enabledApis["run.googleapis.com"],
        registry,
        runSa,
      ],
      lifecycle: {
        // Image is managed by CI/CD (deploy.yml), not CDKTF.
        // Without this, every `cdktf deploy` would reset the image to the placeholder.
        ignoreChanges: ["template[0].containers[0].image"],
      },
    });

    // Allow unauthenticated (public) access
    new CloudRunV2ServiceIamMember(this, "service-public", {
      project: PROJECT,
      location: REGION,
      name: cloudRunService.name,
      role: "roles/run.invoker",
      member: "allUsers",
    });

    // --- Log retention: 30 days ---
    new LoggingProjectBucketConfig(this, "log-retention", {
      project: PROJECT,
      location: "global",
      bucketId: "_Default",
      retentionDays: 30,
    });

    // --- Workload Identity Federation for GitHub Actions ---
    const wifPool = new IamWorkloadIdentityPool(this, "github-pool", {
      workloadIdentityPoolId: "github-pool",
      displayName: "GitHub Actions Pool",
      dependsOn: [enabledApis["iam.googleapis.com"]],
    });

    const wifProvider = new IamWorkloadIdentityPoolProvider(this, "github-provider", {
      workloadIdentityPoolId: wifPool.workloadIdentityPoolId,
      workloadIdentityPoolProviderId: "github-provider",
      displayName: "GitHub OIDC Provider",
      oidc: {
        issuerUri: "https://token.actions.githubusercontent.com",
      },
      attributeMapping: {
        "google.subject": "assertion.sub",
        "attribute.repository": "assertion.repository",
        "attribute.actor": "assertion.actor",
      },
      attributeCondition: `attribute.repository == "${GITHUB_REPO}"`,
    });

    const githubSa = new ServiceAccount(this, "github-sa", {
      accountId: "github-actions-sa",
      displayName: "GitHub Actions Service Account",
    });

    const githubSaRoles = [
      "roles/run.admin",
      "roles/artifactregistry.admin",
      "roles/logging.admin",
      "roles/serviceusage.serviceUsageAdmin",
      "roles/iam.serviceAccountAdmin",
      "roles/resourcemanager.projectIamAdmin",
      "roles/iam.workloadIdentityPoolAdmin",
    ];
    for (const role of githubSaRoles) {
      new ProjectIamMember(this, `github-sa-${role.replace(/\//g, "-")}`, {
        project: PROJECT,
        role,
        member: `serviceAccount:${githubSa.email}`,
      });
    }

    // Allow GitHub Actions SA to act as Cloud Run SA
    new ProjectIamMember(this, "github-sa-act-as-run-sa", {
      project: PROJECT,
      role: "roles/iam.serviceAccountUser",
      member: `serviceAccount:${githubSa.email}`,
    });

    // Allow WIF to impersonate the GitHub Actions SA
    new ServiceAccountIamMember(this, "github-wif-binding", {
      serviceAccountId: githubSa.name,
      role: "roles/iam.workloadIdentityUser",
      member: `principalSet://iam.googleapis.com/${wifPool.name}/attribute.repository/${GITHUB_REPO}`,
    });

    // --- Outputs ---
    new TerraformOutput(this, "cloud-run-url", {
      value: cloudRunService.uri,
      description: "Cloud Run service URL",
    });

    new TerraformOutput(this, "image-base", {
      value: IMAGE_BASE,
      description: "Artifact Registry image base path",
    });

    new TerraformOutput(this, "wif-provider", {
      value: wifProvider.name,
      description: "Workload Identity Provider resource name (use as GCP_WORKLOAD_IDENTITY_PROVIDER secret)",
    });

    new TerraformOutput(this, "github-sa-email", {
      value: githubSa.email,
      description: "GitHub Actions SA email (use as GCP_SERVICE_ACCOUNT_EMAIL secret)",
    });
  }
}

const app = new App();
new TalentsStack(app, "talents");
app.synth();
