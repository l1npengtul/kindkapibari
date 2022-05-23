use kindkapibari_proc::AttrString;
use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};
use thiserror::Error;

const ALLOWED_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

#[derive(Debug, Error)]
enum ParseTagError {
    #[error("Failed to parse {0}")]
    FailToParse(String),
}

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum GenderMarker {
    General,
    Masc,
    Fem,
    Andro,
}

impl Debug for GenderMarker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GenderMarker::General => "General",
                GenderMarker::Masc => "Masculine",
                GenderMarker::Fem => "Feminine",
                GenderMarker::Andro => "Androgynous",
            }
        )
    }
}

impl Display for GenderMarker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for GenderMarker {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Masculine" => Ok(GenderMarker::Masc),
            "Feminine" => Ok(GenderMarker::Fem),
            "Androgynous" => Ok(GenderMarker::Andro),
            "General" => Ok(GenderMarker::General),
            _ => Err(ParseTagError::FailToParse(s.to_string())),
        }
    }
}

// i should write a macro
// no, i definately shouldhave written a macro
// but now i must suffer
#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Jobs {
    // Customer Facing / Selling Stuff / Emotional Labour / Delivery
    Retail,
    Driver,
    Delivery,
    Postman,
    Therapist,
    Janitor,
    CustomerService, // Gen
    WaitStaff,
    // Food Industry
    FastFood,
    Baker,
    Butcher,
    Chef,
    // Engineering
    ElectricalEngineer,
    MechanicalEngineer,
    ChemicalEngineer,
    HardwareEngineer,
    Programmer,
    Engineer, // Gen
    // Creatives
    ContentCreator,
    LiveStreamer,
    Entertainer,
    GraphicDesigner,
    Artist,
    Editor,
    Animator,
    Journalist,
    Cartoonist,
    Performer,
    Writer,
    Musician,
    Poet,
    Dancer,
    Singer,
    Woodcarver,
    Creator, // Gen
    //
    Activist,
    //
    Clerk,
    Secretary,
    Custodian,
    Accountant,
    Manager,
    OfficeWorker, // gen
    //
    Doctor,
    Nurse,
    Paramedic,
    LabWorker,
    Researcher,
    Chemist,
    Biologist,
    Physicist,
    Mathematician,
    Astronaut,
    Astronomer,
    Geologist,
    Psychologist,
    Sociologist,
    SocialScientist,
    Lawyer,
    Judge,
    Professor,
    Linguist,
    Teacher,
    Pharmacist,
    Librarian,
    Educator,
    Calligrapher,
    Scientist, // gen
    //
    Construction,
    Carpenter,
    Plumber,
    Electrician,
    Locksmith,
    Doorman,
    Welder,
    Technician,
    Gardener,
    //
    Athlete,
    Firefighter,
    SocialWorker, // destiny is scared rn
    //
    Farmer,
    Fisher,
    //
    Other(String),
}

impl Debug for Jobs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Jobs::Retail => "Retail",
                Jobs::Driver => "Driver",
                Jobs::Delivery => {
                    "Delivery"
                }
                Jobs::Postman => {
                    "Postman"
                }
                Jobs::Therapist => {
                    "Therapist"
                }
                Jobs::Janitor => {
                    "Janitor"
                }
                Jobs::CustomerService => {
                    "CustomerService"
                }
                Jobs::WaitStaff => {
                    "WaitStaff"
                }
                Jobs::FastFood => {
                    "FastFood"
                }
                Jobs::Baker => {
                    "Baker"
                }
                Jobs::Butcher => {
                    "Butcher"
                }
                Jobs::Chef => {
                    "Chef"
                }
                Jobs::ElectricalEngineer => {
                    "ElectricalEngineer"
                }
                Jobs::MechanicalEngineer => {
                    "MechanicalEngineer"
                }
                Jobs::ChemicalEngineer => {
                    "ChemicalEngineer"
                }
                Jobs::HardwareEngineer => {
                    "HardwareEngineer"
                }
                Jobs::Programmer => {
                    "Programmer"
                }
                Jobs::Engineer => {
                    "Engineer"
                }
                Jobs::ContentCreator => {
                    "ContentCreator"
                }
                Jobs::LiveStreamer => {
                    "LiveStreamer"
                }
                Jobs::Entertainer => {
                    "Entertainer"
                }
                Jobs::GraphicDesigner => {
                    "GraphicDesigner"
                }
                Jobs::Artist => {
                    "Artist"
                }
                Jobs::Editor => {
                    "Editor"
                }
                Jobs::Animator => {
                    "Animator"
                }
                Jobs::Journalist => {
                    "Journalist"
                }
                Jobs::Cartoonist => {
                    "Cartoonist"
                }
                Jobs::Performer => {
                    "Performer"
                }
                Jobs::Writer => {
                    "Writer"
                }
                Jobs::Musician => {
                    "Musician"
                }
                Jobs::Poet => {
                    "Poet"
                }
                Jobs::Dancer => {
                    "Dancer"
                }
                Jobs::Singer => {
                    "Singer"
                }
                Jobs::Woodcarver => {
                    "Woodcarver"
                }
                Jobs::Creator => {
                    "Creator"
                }
                Jobs::Activist => {
                    "Activist"
                }
                Jobs::Clerk => {
                    "Clerk"
                }
                Jobs::Secretary => {
                    "Secretary"
                }
                Jobs::Custodian => {
                    "Custodian"
                }
                Jobs::Accountant => {
                    "Accountant"
                }
                Jobs::Manager => {
                    "Manager"
                }
                Jobs::OfficeWorker => {
                    "OfficeWorker"
                }
                Jobs::Doctor => {
                    "Doctor"
                }
                Jobs::Nurse => {
                    "Nurse"
                }
                Jobs::Paramedic => {
                    "Paramedic"
                }
                Jobs::LabWorker => {
                    "LabWorker"
                }
                Jobs::Researcher => {
                    "Researcher"
                }
                Jobs::Chemist => {
                    "Chemist"
                }
                Jobs::Biologist => {
                    "Biologist"
                }
                Jobs::Physicist => {
                    "Physicist"
                }
                Jobs::Mathematician => {
                    "Mathematician"
                }
                Jobs::Astronaut => {
                    "Astronaut"
                }
                Jobs::Astronomer => {
                    "Astronomer"
                }
                Jobs::Geologist => {
                    "Geologist"
                }
                Jobs::Psychologist => {
                    "Psychologist"
                }
                Jobs::Sociologist => {
                    "Sociologist"
                }
                Jobs::SocialScientist => {
                    "SocialScientist"
                }
                Jobs::Lawyer => {
                    "Lawyer"
                }
                Jobs::Judge => {
                    "Judge"
                }
                Jobs::Professor => {
                    "Professor"
                }
                Jobs::Linguist => {
                    "Linguist"
                }
                Jobs::Teacher => {
                    "Teacher"
                }
                Jobs::Pharmacist => {
                    "Pharmacist"
                }
                Jobs::Librarian => {
                    "Librarian"
                }
                Jobs::Educator => {
                    "Educator"
                }
                Jobs::Calligrapher => {
                    "Calligrapher"
                }
                Jobs::Scientist => {
                    "Scientist"
                }
                Jobs::Construction => {
                    "Construction"
                }
                Jobs::Carpenter => {
                    "Carpenter"
                }
                Jobs::Plumber => {
                    "Plumber"
                }
                Jobs::Electrician => {
                    "Electrician"
                }
                Jobs::Locksmith => {
                    "Locksmith"
                }
                Jobs::Doorman => {
                    "Doorman"
                }
                Jobs::Welder => {
                    "Welder"
                }
                Jobs::Technician => {
                    "Technician"
                }
                Jobs::Gardener => {
                    "Gardener"
                }
                Jobs::Athlete => {
                    "Athlete"
                }
                Jobs::Firefighter => {
                    "Firefighter"
                }
                Jobs::SocialWorker => {
                    "SocialWorker"
                }
                Jobs::Farmer => {
                    "Farmer"
                }
                Jobs::Fisher => {
                    "Fisher"
                }
                Jobs::Other(job) => job,
            }
        )
    }
}

impl Display for Jobs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// i want to kill myself
// end my suffering
impl FromStr for Jobs {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Retail" => Jobs::Retail,
            "Driver" => Jobs::Driver,
            "Delivery" => Jobs::Delivery,
            "Postman" => Jobs::Postman,
            "Therapist" => Jobs::Therapist,
            "Janitor" => Jobs::Janitor,
            "CustomerService" => Jobs::CustomerService,
            "WaitStaff" => Jobs::WaitStaff,
            "FastFood" => Jobs::FastFood,
            "Baker" => Jobs::Baker,
            "Butcher" => Jobs::Butcher,
            "Chef" => Jobs::Chef,
            "ElectricalEngineer" => Jobs::ElectricalEngineer,
            "MechanicalEngineer" => Jobs::MechanicalEngineer,
            "ChemicalEngineer" => Jobs::ChemicalEngineer,
            "HardwareEngineer" => Jobs::HardwareEngineer,
            "Programmer" => Jobs::Programmer,
            "Engineer" => Jobs::Engineer,
            "ContentCreator" => Jobs::ContentCreator,
            "LiveStreamer" => Jobs::LiveStreamer,
            "Entertainer" => Jobs::Entertainer,
            "GraphicDesigner" => Jobs::GraphicDesigner,
            "Artist" => Jobs::Artist,
            "Editor" => Jobs::Editor,
            "Animator" => Jobs::Animator,
            "Journalist" => Jobs::Journalist,
            "Cartoonist" => Jobs::Cartoonist,
            "Performer" => Jobs::Performer,
            "Writer" => Jobs::Writer,
            "Musician" => Jobs::Musician,
            "Poet" => Jobs::Poet,
            "Dancer" => Jobs::Dancer,
            "Singer" => Jobs::Singer,
            "Woodcarver" => Jobs::Woodcarver,
            "Creator" => Jobs::Creator,
            "Activist" => Jobs::Activist,
            "Clerk" => Jobs::Doctor,
            "Secretary" => Jobs::Secretary,
            "Custodian" => Jobs::Custodian,
            "Accountant" => Jobs::Accountant,
            "Manager" => Jobs::Manager,
            "OfficeWorker" => Jobs::OfficeWorker,
            "Doctor" => Jobs::Doctor,
            "Nurse" => Jobs::Nurse,
            "Paramedic" => Jobs::Paramedic,
            "LabWorker" => Jobs::LabWorker,
            "Researcher" => Jobs::Researcher,
            "Chemist" => Jobs::Chemist,
            "Biologist" => Jobs::Biologist,
            "Physicist" => Jobs::Physicist,
            "Mathematician" => Jobs::Mathematician,
            "Astronaut" => Jobs::Astronaut,
            "Astronomer" => Jobs::Astronomer,
            "Geologist" => Jobs::Geologist,
            "Psychologist" => Jobs::Psychologist,
            "Sociologist" => Jobs::Sociologist,
            "SocialScientist" => Jobs::SocialScientist,
            "Lawyer" => Jobs::Lawyer,
            "Judge" => Jobs::Judge,
            "Professor" => Jobs::Professor,
            "Linguist" => Jobs::Linguist,
            "Teacher" => Jobs::Teacher,
            "Pharmacist" => Jobs::Pharmacist,
            "Librarian" => Jobs::Librarian,
            "Educator" => Jobs::Educator,
            "Calligrapher" => Jobs::Calligrapher,
            "Scientist" => Jobs::Scientist,
            "Construction" => Jobs::Construction,
            "Carpenter" => Jobs::Carpenter,
            "Plumber" => Jobs::Plumber,
            "Electrician" => Jobs::Electrician,
            "Locksmith" => Jobs::Locksmith,
            "Doorman" => Jobs::Doorman,
            "Welder" => Jobs::Welder,
            "Technician" => Jobs::Technician,
            "Gardener" => Jobs::Gardener,
            "Athlete" => Jobs::Athlete,
            "Firefighter" => Jobs::Firefighter,
            "SocialWorker" => Jobs::SocialWorker,
            "Farmer" => Jobs::Farmer,
            "Fisher" => Jobs::Fisher,
            job => {
                if job.replace(ALLOWED_CHARS, "") != "" {
                    return Err(ParseTagError::FailToParse("Bad Characters".to_string()));
                }
                job
            }
        })
    }
}

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Pets {
    Dog,
    Cat,
    Fish,
    Mouse,
    GuineaPig,
    Bird,
    Snake,
    Iguana,
    Turtle,
    Ferret,
    Capybara,
    Frog,
    Ant,
    Gecko,
    Goat,
    Pig,
    Chicken,
    Cow, // chlopeing right now
    Hamster,
    Horse, // vowsh-san will you take the estrogen pill for me :pleading_face:
    HermitCrab,
    Crab,
    Spider,
    Insect,
    Lizard,
    Repitle,
    Rabbit,
    Duck,
    Snail,
    Other(String),
}

impl Debug for Pets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pets::Dog => "Dog",
                Pets::Cat => "Cat",
                Pets::Fish => "Fish",
                Pets::Mouse => "Mouse",
                Pets::GuineaPig => "GuineaPig",
                Pets::Bird => "Bird",
                Pets::Snake => "Snake",
                Pets::Iguana => "Iguana",
                Pets::Turtle => "Turtle",
                Pets::Ferret => "Ferret",
                Pets::Capybara => "Capybara",
                Pets::Frog => "Frog",
                Pets::Ant => "Ant",
                Pets::Gecko => "Gecko",
                Pets::Goat => "Goat",
                Pets::Pig => "Pig",
                Pets::Chicken => "Chicken",
                Pets::Cow => "Cow",
                Pets::Hamster => "Hamster",
                Pets::Horse => "Horse",
                Pets::HermitCrab => "HermitCrab",
                Pets::Crab => "Crab",
                Pets::Spider => "Spider",
                Pets::Insect => "Insect",
                Pets::Lizard => "Lizard",
                Pets::Repitle => "Repitle",
                Pets::Rabbit => "Rabbit",
                Pets::Duck => "Duck",
                Pets::Snail => "Snail",
                Pets::Other(pet) => pet,
            }
        )
    }
}

impl Display for Pets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Pets {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Dog" => Pets::Dog,
            "Cat" => Pets::Cat,
            "Fish" => Pets::Fish,
            "Mouse" => Pets::Mouse,
            "GuineaPig" => Pets::GuineaPig,
            "Bird" => Pets::Bird,
            "Snake" => Pets::Snake,
            "Iguana" => Pets::Iguana,
            "Turtle" => Pets::Turtle,
            "Ferret" => Pets::Ferret,
            "Capybara" => Pets::Capybara,
            "Frog" => Pets::Frog,
            "Ant" => Pets::Ant,
            "Gecko" => Pets::Goat,
            "Goat" => Pets::Goat,
            "Pig" => Pets::Pig,
            "Chicken" => Pets::Chicken,
            "Cow" => Pets::Cat,
            "Hamster" => Pets::Hamster,
            "Horse" => Pets::Horse,
            "HermitCrab" => Pets::HermitCrab,
            "Crab" => Pets::Crab,
            "Spider" => Pets::Spider,
            "Insect" => Pets::Insect,
            "Lizard" => Pets::Lizard,
            "Repitle" => Pets::Repitle,
            "Rabbit" => Pets::Rabbit,
            "Duck" => Pets::Duck,
            "Snail" => Pets::Snail,
            pet => {
                if pet.replace(ALLOWED_CHARS, "") != "" {
                    return Err(ParseTagError::FailToParse("Bad Characters".to_string()));
                }
                pet
            }
        })
    }
}

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Tags {
    // General Interest Tags
    Advice,
    Wholesome,
    Profession(Jobs),
    Interests(Jobs),
    Pets(Pets),
    Anime,
    Cartoons,
    Music,
    Shows,
    Movies,
    Games,
    Videos,
    Academics,
    Humor,
    Health,
    // Mental Health
    Depression,
    MentalHealth,
    // Trans/Gender related
    Fem,
    TransFem,
    CisFem,
    Masc,
    TransMasc,
    CisMasc,
    NonBinary,
    Androgynous,
    Femboy,
    Tomboy,
    XenoGender(String),
    VoiceTraining(GenderMarker),
    Dysphoria(GenderMarker),
    Transition(GenderMarker),
    Presentation(GenderMarker),
    DiyHrt(GenderMarker),
    Hrt(GenderMarker),
    // (based) politics
    AntiTerf,
    Progressive,
    RecentEvents,
    //
    Other(String),
}

impl Debug for Tags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // TODO: is there a way to do these without these allocs?
                Tags::Fem => "Feminine",
                Tags::TransFem => "TransFem",
                Tags::CisFem => "CisFem",
                Tags::Masc => "Masculine",
                Tags::TransMasc => "TransMasc",
                Tags::CisMasc => "CisMasc",
                Tags::NonBinary => "NonBinary",
                Tags::Androgynous => "Androgynous",
                Tags::XenoGender(gendr) => &format!("XenoGender({gendr})"),
                Tags::VoiceTraining(gm) => &format!("VoiceTraining({gm})"),
                Tags::Dysphoria(gm) => &format!("Dysphoria({gm})"),
                Tags::Profession(job) => &format!("Profession({job})"),
                Tags::Transition(gm) => &format!("Transition({gm})"),
                Tags::Presentation(gm) => &format!("Presentation({gm})"),
                Tags::AntiTerf => "AntiTerf",
                Tags::Progressive => "Progressive",
                Tags::Pets(pet) => &format!("Pets({pet})"),
                Tags::Advice => "Advice",
                Tags::Wholesome => "Wholesome",
                Tags::Interests(i) => &format!("Interests({i})"),
                Tags::Anime => "Anime",
                Tags::Music => "Music",
                Tags::Shows => "Shows",
                Tags::Movies => "Movies",
                Tags::Games => "Games",
                Tags::Videos => "Videos",
                Tags::Academics => "Academics",
                Tags::Humor => "Humor",
                Tags::Health => "Health",
                Tags::Depression => "Depression",
                Tags::MentalHealth => "MentalHealth",
                Tags::Femboy => "Femboy",
                Tags::Tomboy => "Tomboy",
                Tags::DiyHrt(gm) => &format!("DIYHRT({gm})"),
                Tags::Hrt(gm) => &format!("HRT({gm})"),
                Tags::RecentEvents => "RecentEvents",
                Tags::Cartoons => "Cartoons",
                Tags::Other(o) => &o,
            }
        )
    }
}

impl FromStr for Tags {
    type Err = ParseTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.replace([ALLOWED_CHARS, &['(', ')']].concat(), "") != "" {
            return Err(ParseTagError::FailToParse("Bad Characters".to_string()));
        }
        let splitted = s.split(&['(', ')']).collect::<Vec<String>>();
        if splitted.len() == 1 {
            let t = &splitted[0];
            Ok(match t.as_str() {
                "Advice" => Tags::Advice,
                "Wholesome" => Tags::Wholesome,
                "Anime" => Tags::Anime,
                "Cartoons" => Tags::Cartoons,
                "Music" => Tags::Music,
                "Shows" => Tags::Shows,
                "Movies" => Tags::Movies,
                "Games" => Tags::Games,
                "Videos" => Tags::Videos,
                "Academics" => Tags::Academics,
                "Humor" => Tags::Humor,
                "Health" => Tags::Health,
                "Depression" => Tags::Depression,
                "MentalHealth" => Tags::MentalHealth,
                "Fem" => Tags::Fem,
                "TransFem" => Tags::TransFem,
                "CisFem" => Tags::CisFem,
                "Masc" => Tags::Masc,
                "TransMasc" => Tags::TransMasc,
                "CisMasc" => Tags::CisFem,
                "NonBinary" => Tags::NonBinary,
                "Androgynous" => Tags::Androgynous,
                "Femboy" => Tags::Femboy,
                "AntiTerf" => Tags::AntiTerf,
                "Progressive" => Tags::Progressive,
                "RecentEvents" => Tags::RecentEvents,
                tag => Tags::Other(tag.to_string()), //
            })
        } else if splitted.len() == 2 {
            let t = &splitted[0];
            let subtag = &splitted[1];
            Ok(match t.as_str() {
                "Profession" => Tags::Profession(subtag.parse::<Jobs>()?),
                "Interests" => Tags::Interests(subtag.parse::<Jobs>()?),
                "Pets" => Tags::Pets(subtag.parse::<Pets>()?),
                "XenoGender" => Tags::XenoGender(subtag.to_string()),
                "VoiceTraining" => Tags::VoiceTraining(subtag.parse::<GenderMarker>()?),
                "Dysphoria" => Tags::Dysphoria(subtag.parse::<GenderMarker>()?),
                "Transition" => Tags::Transition(subtag.parse::<GenderMarker>()?),
                "Presentation" => Tags::Presentation(subtag.parse::<GenderMarker>()?),
                "DiyHrt" => Tags::DiyHrt(subtag.parse::<GenderMarker>()?),
                "Hrt" => Tags::Hrt(subtag.parse::<GenderMarker>()?),
                _ => return Err(ParseTagError::FailToParse("Bad Characters.".to_string())),
            })
        } else {
            Err(ParseTagError::FailToParse(
                "Bad Tag (Empty or Invalid)".to_json_string(),
            ))
        }
    }
}
