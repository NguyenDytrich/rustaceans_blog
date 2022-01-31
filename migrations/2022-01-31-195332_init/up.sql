CREATE TABLE blog_posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) UNIQUE NOT NULL,
  body TEXT,
  /* Use current time if no timestamp is provided */
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  is_public BOOLEAN DEFAULT false NOT NULL
);