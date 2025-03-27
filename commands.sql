DROP TABLE People;
DROP TABLE Transactions;

CREATE TABLE People (
    TransactionID INTEGER PRIMARY KEY AUTOINCREMENT,
    Symbol Varchar(255),
    Name Varchar(255)
);

INSERT INTO People (Symbol, Name)
VALUES ('mah', 'Aayan'),
       ('arl', 'Amelio'),
       ('bib', 'Bibek'),
       ('geh', 'Gabriel'),
       ('lwg', 'London'),
       ('j', 'Jai'),
       ('jjb', 'Josh'),
       ('nf', 'Naumaan'),
       ('ss', 'Steven');

CREATE TABLE Transactions (
    TransactionID INTEGER PRIMARY KEY AUTOINCREMENT,
    Buyer VARCHAR(255),
    Seller VARCHAR(255),
    Creator VARCHAR(255),
    Symbol VARCHAR(255),
    Cents UNSIGNED INT,
    Quantity UNSIGNED INT,
    Presented UNSIGNED BIG INT,
    Sold UNSIGNED UNSIGNED BIG INT
);

INSERT INTO Transactions (Buyer, Seller, Creator, Symbol, Cents, Quantity, Presented, Sold)
Values ('mah', 'j', 'bib', 'ss', 321, 2314, 12, 1743115377),
       ('arl', 'ss', 'nf', 'geh', 324, 123, 32, 1743115377);
