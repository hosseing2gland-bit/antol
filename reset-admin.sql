-- Reset admin user with correct password
DELETE FROM users WHERE email = 'admin@demo.com';

INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) 
VALUES (
  gen_random_uuid(),
  'admin@demo.com',
  '$2b$12$yuj/WR0m7wKFH7Q0/FLik.5mWYOz/fmqZIXkBos9Oh3b9iI/ob6fW',
  'admin',
  true,
  NOW(),
  NOW()
);

SELECT 'Admin user created successfully!' as status, email, role FROM users WHERE email = 'admin@demo.com';
