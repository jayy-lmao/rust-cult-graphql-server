CREATE TABLE IF NOT EXISTS cults (
    id serial PRIMARY KEY,
    address varchar(100),
    cult_description text,
    cult_website varchar(100),
    brand_colour varchar(100),
    email varchar(100),
    logo_url varchar(500),
    name varchar(100),
    postcode varchar(5),
    state varchar(100)
);

CREATE TABLE IF NOT EXISTS cultists (
    id serial PRIMARY KEY,
    email varchar(100),
    first_name varchar(100),
    last_name varchar(100),
    mobile_phone varchar(100),
    date_created TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS cultist_cults (
    cultist_id INTEGER,
    cult_id INTEGER,
    PRIMARY KEY (cultist_id, cult_id)
);

CREATE TABLE IF NOT EXISTS rituals (
    Id SERIAL PRIMARY KEY,
    ritual_type varchar(100)
);

-- Sample Data --
INSERT INTO
    cults(
        address,
        cult_description,
        cult_website,
        brand_colour,
        email,
        logo_url,
        name,
        postcode,
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
        first_name,
        last_name,
        email,
        mobile_phone
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
    cultist_cults(cultist_id, cult_id)
VALUES
    (1, 1),
    (2, 1),
    (3, 1),
    (4, 1);

INSERT INTO
    rituals(ritual_type)
VALUES
    ('TarotReading');