CREATE TABLE chat_user_stats (
  chat_id       BIGINT      NOT NULL,
  user_id       BIGINT      NOT NULL,
  message_type  SMALLINT    NOT NULL,
  messages      INT         NOT NULL,
  edits         INT         NOT NULL,
  created_at    DATETIME    NOT NULL
    DEFAULT CURRENT_TIMESTAMP,
  updated_at    DATETIME    NOT NULL
    DEFAULT CURRENT_TIMESTAMP
    ON UPDATE CURRENT_TIMESTAMP,

  PRIMARY KEY (chat_id, user_id, message_type),
  FOREIGN KEY (chat_id)
    REFERENCES chat(telegram_id)
    ON DELETE CASCADE,
  FOREIGN KEY (user_id)
    REFERENCES user(telegram_id)
    ON DELETE CASCADE
);
