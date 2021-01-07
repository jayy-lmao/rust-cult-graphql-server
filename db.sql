CREATE TABLE IF NOT EXISTS cults (
    id serial PRIMARY KEY,
    address varchar(100),
    cultDescription text,
    cultWebsite varchar(100),
    brandColour varchar(100),
    email varchar(100),
    logoUrl varchar(500),
    name varchar(100),
    postCode varchar(5),
    state varchar(100)
);

CREATE TABLE IF NOT EXISTS cultists (
    email varchar(100),
    firstName varchar(100),
    lastName varchar(100),
    mobilePhone varchar(100),
    dateCreated timestamp DEFAULT now(),
    dateDeleted timestamp,
    dateUpdated timestamp
);

CREATE TABLE IF NOT EXISTS cultistCults (
    cultistId INTEGER,
    cultId INTEGER,
    pRIMARY KEY (cultistId, cultId)
);

CREATE TABLE IF NOT EXISTS rituals (
    Id SERIAL PRIMARY KEY,
    ritualType varchar(100)
);

-- Sample Data --
INSERT INTO
    cults(
        address,
        cultDescription,
        cultWebsite,
        brandColour,
        email,
        logoUrl,
        name,
        postCode,
        state
    )
VALUES
    (
        '20 Crowley Crescent',
        'Gotta have fun!',
        'www.shenanigans.com.au',
        '#000000',
        'shenanigans@crowley.edu.au',
        'https://www.otago.ac.nz/english-linguistics/english/lowry/content/11_mysticism/d_people/d_img/DL_crowley.png',
        'The Creepy Crowleys',
        '2051',
        'NSW'
    );

INSERT INTO
    cultists(
        firstName,
        lastName,
        email,
        mobilePhone
    )
VALUES
    (
        'Bob',
        'Bobbington',
        'bobbb@gmail.com',
        '0427987345'
    ),
    (
        'Steve',
        'Marshall',
        'stevem@gmail.com',
        '0437927113'
    ),
    (
        'Jeffrey',
        'Stepstein',
        'jeffrey@island.com',
        '0427487345'
    ),
    (
        'Antonio',
        'Midas',
        'tony@proton.com',
        '0423499874'
    ),
    (
        'Eugenius',
        'A.',
        'Eugene@gmail.com',
        '0483798792'
    );

INSERT INTO
    cultistCults(cultistId, cultId)
VALUES
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0);

INSERT INTO
    rituals(ritualType)
VALUES
    ('TarotReading');