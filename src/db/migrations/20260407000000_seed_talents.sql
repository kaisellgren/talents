-- Seed 1000 realistic fake talents using random array lookups
DO $$
DECLARE
  first_names TEXT[] := ARRAY[
    'Liam','Emma','Noah','Olivia','James','Sophia','Hiroshi','Yuki','Carlos','Sofia',
    'Amara','Kwame','Leila','Omar','Nina','Dmitri','Ingrid','Rafael','Priya','Arjun',
    'Mei','Zhang','Fatima','Ade','Sven','Elsa','Marco','Giulia','Andrei','Nadia',
    'Lucas','Isabelle','Kenji','Hana','Diego','Valentina','Finn','Astrid','Rohan','Anika'
  ];
  last_names TEXT[] := ARRAY[
    'Smith','Garcia','Müller','Tanaka','Okafor','Dubois','Patel','Nakamura','Silva','Johansson',
    'Kim','Wang','Fernandez','Rossi','Ivanov','Becker','Nguyen','Hassan','Cohen','Reyes',
    'Brown','Andersen','Fischer','Yamamoto','Diallo','Martins','Petrov','Larsson','Mehta','Öztürk',
    'Wilson','Martinez','Bergmann','Park','Mensah','Santos','Kowalski','Nielsen','Sharma','Lindqvist'
  ];
  -- locations: city, country, state (NULL where not applicable)
  loc_cities    TEXT[] := ARRAY[
    'San Francisco','New York','Austin','Seattle','Boston','Chicago','Los Angeles','Denver','Atlanta','Miami',
    'London','Berlin','Amsterdam','Paris','Stockholm','Zurich','Dublin','Barcelona','Warsaw','Vienna',
    'Toronto','Vancouver','Sydney','Singapore','Tokyo','Seoul','Bangalore','São Paulo','Mexico City','Cape Town'
  ];
  loc_countries TEXT[] := ARRAY[
    'United States','United States','United States','United States','United States','United States','United States','United States','United States','United States',
    'United Kingdom','Germany','Netherlands','France','Sweden','Switzerland','Ireland','Spain','Poland','Austria',
    'Canada','Canada','Australia','Singapore','Japan','South Korea','India','Brazil','Mexico','South Africa'
  ];
  roles TEXT[] := ARRAY[
    'Software Engineer','Backend Developer','Frontend Developer','Full Stack Developer',
    'Data Scientist','ML Engineer','DevOps Engineer','Security Engineer',
    'Product Manager','UX Designer','QA Engineer','Engineering Manager'
  ];

  -- Skills grouped loosely; we use a single flat pool and pick randomly
  all_skills TEXT[] := ARRAY[
    'rust','go','python','typescript','javascript','java','c++','c#','kotlin','swift',
    'react','vue','angular','next.js','svelte','node.js','fastapi','django','spring boot','asp.net',
    'postgresql','mysql','mongodb','redis','elasticsearch','cassandra','dynamodb','sqlite',
    'aws','gcp','azure','docker','kubernetes','terraform','ansible','pulumi',
    'graphql','rest','grpc','kafka','rabbitmq','prometheus','grafana','opentelemetry',
    'machine learning','pytorch','tensorflow','scikit-learn','pandas','spark','dbt','airflow',
    'figma','sketch','cypress','playwright','junit','pytest','github actions','jenkins'
  ];

  biographies TEXT[] := ARRAY[
    'Passionate engineer with a focus on scalable distributed systems and developer experience.',
    'Full-stack craftsperson who enjoys turning complex problems into elegant solutions.',
    'Data-driven professional who loves building models that make a real business impact.',
    'Infrastructure enthusiast obsessed with reliability, automation, and zero-downtime deployments.',
    'Design-minded developer who bridges the gap between product vision and technical execution.',
    'Security-first engineer with experience in threat modelling, pen testing, and secure SDLC.',
    'Collaborative engineering manager who cares deeply about team health and sustainable pace.',
    'Backend specialist with a knack for high-throughput APIs and event-driven architectures.',
    'Frontend artisan who sweats the details of performance, accessibility, and delightful UX.',
    'ML engineer translating cutting-edge research into production-ready machine learning pipelines.',
    'Product manager who speaks fluent engineering and loves shipping with cross-functional teams.',
    'QA engineer who believes quality is everyone''s job and automation is the path to confidence.'
  ];

  n_first  INT := array_length(first_names, 1);
  n_last   INT := array_length(last_names, 1);
  n_loc    INT := array_length(loc_cities, 1);  -- loc_cities and loc_countries are parallel arrays
  n_roles  INT := array_length(roles, 1);
  n_skills INT := array_length(all_skills, 1);
  n_bios   INT := array_length(biographies, 1);

  i         INT;
  skill_cnt INT;
  rate_min  INT;
  rate_max  INT;
  loc_idx   INT;
  shuffled  TEXT[];
BEGIN
  FOR i IN 1..1000 LOOP
    loc_idx   := floor(random() * n_loc + 1)::INT;
    skill_cnt := floor(random() * 4 + 3)::INT;   -- 3–6 skills
    rate_min  := floor(random() * 101 + 50)::INT; -- 50–150
    rate_max  := rate_min + floor(random() * 51 + 20)::INT; -- min+20 to min+70

    -- Shuffle all_skills and take first skill_cnt elements
    SELECT array_agg(s ORDER BY random())
    INTO shuffled
    FROM unnest(all_skills) s;

    INSERT INTO talents (
      name,
      location_city,
      location_country,
      role,
      skills,
      available,
      hourly_rate_min,
      hourly_rate_max,
      biography
    ) VALUES (
      first_names[floor(random() * n_first + 1)::INT] || ' ' ||
        last_names[floor(random() * n_last + 1)::INT],
      loc_cities[loc_idx],
      loc_countries[loc_idx],
      roles[floor(random() * n_roles + 1)::INT],
      (SELECT jsonb_agg(s) FROM unnest(shuffled[1:skill_cnt]) s),
      random() > 0.4,
      rate_min,
      rate_max,
      biographies[floor(random() * n_bios + 1)::INT]
    );
  END LOOP;
END $$;
