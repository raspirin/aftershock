-- Your SQL goes here
CREATE TABLE contents_tags (
  content_id INTEGER NOT NULL REFERENCES contents(id),
  tag_id INTEGER NOT NULL REFERENCES tags(id),
  PRIMARY KEY(content_id, tag_id)
);
