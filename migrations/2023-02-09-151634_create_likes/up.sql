-- Your SQL goes here
CREATE TABLE IF NOT EXISTS likes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    tweet_id INT NOT NULL,
    INDEX (tweet_id),
    FOREIGN KEY (tweet_id) REFERENCES tweets(id)
)  ENGINE=INNODB;