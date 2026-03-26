use crate::models::{Minister, Oiv, StaffInfo, StaffMember, Status};
use rand::Rng;
use rand::seq::SliceRandom;
use std::{mem, vec};

pub struct DataSet {
    pub members: Vec<StaffMember>,
    pub oivs: Vec<Oiv>,
}

fn get_organisations() -> Vec<StaffInfo> {
    vec![
        StaffInfo::new("Адептус Астартес"),
        StaffInfo::new("Адепта Сороритас"),
        StaffInfo::new("Адептус Механикус"),
        StaffInfo::new("Империум Человечества"),
        StaffInfo::new("Имперская Гвардия"),
        StaffInfo::new("Инквизиция"),
    ]
}

fn get_products() -> Vec<StaffInfo> {
    vec![
        StaffInfo::new("Варп-двигатель"),
        StaffInfo::new("Золотой трон"),
        StaffInfo::new("Боескафандры"),
        StaffInfo::new("ВОКС оборудование"),
        StaffInfo::new("Сервочереп"),
        StaffInfo::new("Засекречено"),
    ]
}

fn get_divisions() -> Vec<StaffInfo> {
    vec![
        StaffInfo::new("Культ Сервиторов"),
        StaffInfo::new("Культ Навигаторов"),
        StaffInfo::new("Культ Виртус"),
        StaffInfo::new("Культ Кибернетики"),
        StaffInfo::new("Культ Архивов"),
        StaffInfo::new("Культ Скрижалей"),
    ]
}

fn get_locations() -> Vec<StaffInfo> {
    vec![
        StaffInfo::new("Культ Сервиторов"),
        StaffInfo::new("Культ Навигаторов"),
        StaffInfo::new("Культ Виртус"),
        StaffInfo::new("Культ Кибернетики"),
        StaffInfo::new("Культ Архивов"),
        StaffInfo::new("Культ Скрижалей"),
    ]
}

fn get_addresses() -> Vec<StaffInfo> {
    vec![
        StaffInfo::new("Armageddon Prime, Hive Tartarus, Level 87, Block K-19-Theta"),
        StaffInfo::new("Mars, Forge Manufactorum IX, Sub-Level 204-B, Cogitator Vault 7"),
        StaffInfo::new(
            "Yakov-Apostol Prime, Plasma-Forges Delta, Turbine Chamber 56, Servitor Bay 312",
        ),
        StaffInfo::new("Terra, Holy Terra, Senatorum Spire, Level 999, Apartment Sanctus-42"),
        StaffInfo::new("Cad ia, Fortress Cadian Gate, Barracks Sector 14, Platoon Quarters Gamma"),
        StaffInfo::new("Commorragh, Dark Eldar Spire, Torture-Pit XIV, Slave-Quarters Omega"),
        StaffInfo::new("Cadia, Kasr Vortan, Siege Bastion Delta-9, Shrine of the Emperor’s Wrath"),
        StaffInfo::new("Holy Terra, Administratum Spire 777, Lex Chamber Primus, Data Sanctum IV"),
        StaffInfo::new("Armageddon, Hive Helsreach, Manufactorum 12-Gamma, Smog Vent District"),
        StaffInfo::new("Fenris, Ice Plateau Vargard, Space Wolves Fang, Hall of Echoes"),
        StaffInfo::new("Macragge, Fortress of Hera, Ultramar Bastion, Honor Hall Omega"),
        StaffInfo::new("Nocturne, City of Hesiod, Forge Temple XI, Promethium Catacombs"),
        StaffInfo::new("Necromunda, Underhive Sec-42, Corpse Grinders’ Den, Reclamation Pit Theta"),
        StaffInfo::new("Commorragh, High Spire Zarakthul,Execution Balcony IX"),
        StaffInfo::new("Ba’al, Blood Angels Monastery, Reliquary of Sanguinius, Crypt Level XIII"),
        StaffInfo::new("Medusa, Forge-Spire Tertius, Iron Hands Foundry, Sector Ferrum-9"),
        StaffInfo::new("Catachan, Deathworld Basin Alpha, Jungle Outpost, Venom Grove Station"),
        StaffInfo::new("Mars, Noctis Labyrinthus, Forge Temple Omicron, Data Forge 23-Red"),
        StaffInfo::new("Prospero, City of Tizca, Obsidian Spire, Library of the Pyrae"),
        StaffInfo::new("Krieg, Siege Zone 445, Bunker Complex Sigma-Prime, Trenches of Faith"),
        StaffInfo::new("Cadia Ruins, Kasr Myrak, Warped Bastion, Chaos Shrine of Abaddon"),
        StaffInfo::new("Tanith, Ghost Regiment Camp, Forest Sector 9, Shade Encampment"),
        StaffInfo::new("Terra, Ecclesiarchal Palace, Basilica Mortis, Catacomb Wing Alpha"),
        StaffInfo::new("Vostroya, Firstborn Quarter, Manufactorum District, Plasma Refinery Beta"),
        StaffInfo::new("Cadia Fracture, Warp Rift Zone 13, Blackstone Fragment"),
    ]
}

fn get_names() -> Vec<String> {
    vec![
        "Фулгрим",
        "Логан",
        "Абаддон",
        "Корвус",
        "Ариман",
        "Данте",
        "Велиал",
        "Индрик",
        "Саммаил",
        "Кайваан",
        "Ревюэль",
        "Торквемада",
        "Леман",
        "Мортарион",
        "Магнус",
        "Хорус",
        "Сангвиний",
        "Ангрон",
        "Гарро",
        "Локен",
        "Аргел",
        "Саймон",
        "Талин",
        "Дак",
        "Скорн",
        "Заал",
        "Корон",
        "Кавик",
        "Гальва",
        "Эриан",
        "Тиран",
        "Древар",
        "Крис",
        "Ворн",
        "Жак",
        "Лазар",
        "Медвен",
        "Сталон",
        "Варис",
        "Крон",
        "Релан",
        "Габриэль",
        "Натан",
        "Исаак",
        "Меле",
        "Пэтрик",
        "Сиран",
        "Фенрис",
        "Талас",
        "Ксар",
        "Захар",
        "Браан",
        "Эго",
        "Торка",
        "Фарен",
        "Вейл",
        "Каллен",
        "Роан",
        "Дракен",
        "Свен",
        "Элерон",
        "Нейтан",
        "Орик",
        "Поллак",
        "Кидан",
        "Терон",
        "Вирак",
        "Серис",
        "Лайрик",
        "Корвин",
        "Тевар",
        "Зейн",
        "Ирис",
        "Ренар",
        "Калиус",
        "Майрен",
        "Галас",
        "Вилан",
        "Диксон",
        "Нерон",
        "Эндекс",
        "Тарос",
        "Карен",
        "Лиан",
        "Юрик",
        "Вентрис",
        "Делиан",
        "Сайрус",
        "Керн",
        "Омар",
        "Филан",
        "Эрик",
        "Сурен",
        "Ринас",
        "Дорен",
        "Марек",
        "Жан",
        "Винрель",
        "Сарен",
        "Элвин",
        "Тандор",
        "Грин",
        "Вилар",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn get_surnames() -> Vec<String> {
    vec![
        "Гримнар",
        "Разоритель",
        "Коракс",
        "Красный",
        "Тигурий",
        "Вентан",
        "Арвида",
        "Котэс",
        "Страйфсон",
        "Шрайк",
        "Аргел",
        "Вентрис",
        "Мортарион",
        "Рем",
        "Зелл",
        "Гримальдус",
        "Таранис",
        "Шакал",
        "Талос",
        "Малхарион",
        "Бетанкор",
        "Рун",
        "Кёрз",
        "Тигурий",
        "Вардус",
        "Литтл",
        "Флойд",
        "Нокс",
        "Драк",
        "Кроун",
        "Локен",
        "Севатар",
        "Руна",
        "Вулф",
        "Терор",
        "Голдинг",
        "Дракол",
        "Норен",
        "Веланид",
        "Карзиан",
        "Трос",
        "Фаррелл",
        "Виндоу",
        "Прайм",
        "Мерваллион",
        "Алонсо",
        "Сторм",
        "Грей",
        "Дюран",
        "Кортез",
        "Гордан",
        "Варден",
        "Нортон",
        "Роллинс",
        "Грейвс",
        "Вест",
        "Тарион",
        "Каннингем",
        "Черч",
        "Барретт",
        "Грант",
        "Рейн",
        "Дарк",
        "Фрост",
        "Кэрролл",
        "Макгрегор",
        "Кросс",
        "Фергюсон",
        "Холл",
        "Элвин",
        "Фарроу",
        "Блэк",
        "Олден",
        "Серил",
        "Драммонд",
        "Брукс",
        "Сиверс",
        "Каллис",
        "Грейв",
        "Хант",
        "Флеминг",
        "Брент",
        "Уоллес",
        "Картер",
        "Дрейк",
        "Ламбер",
        "Ренар",
        "Кинкейд",
        "Макконнелл",
        "Гилберт",
        "Сандерс",
        "Перри",
        "Киллер",
        "Рандольф",
        "Фаулер",
        "Харрисон",
        "Прайс",
        "Томлинсон",
        "Эймс",
        "Клейтон",
        "Лафлин",
        "Харпер",
        "Кармайкл",
        "Уитман",
        "Джарретт",
        "Нэш",
        "Бишоп",
        "Макаллистер",
        "Винсент",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn get_occupation() -> Vec<String> {
    vec![
        "Инквизитор",
        "Вольный торговец",
        "Комиссар",
        "Космодесантник",
        "Апотекарий",
        "Техномаг",
        "Навигатор",
        "Астропат",
        "Канонисса",
        "Сестра битвы",
        "Генерал Астра Милитарум",
        "Командир танка",
        "Пилот «Валькирии»",
        "Адепт Администратума",
        "Магос-биолог",
        "Магос-исследователь",
        "Культмеханик-скитарий",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn get_dataset(base_url: &str) -> DataSet {
    let mut members: Vec<StaffMember> = Vec::new();
    let mut oivs: Vec<Oiv> = Vec::new();
    let names = get_names();
    let surnames = get_surnames();
    for i in 0..100 {
        let divided_by_2: bool = i % 2 == 0;
        let divided_by_3: bool = i % 3 == 0;
        let divided_by_4: bool = i % 4 == 0;
        let name: String = names
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or("John Doe".to_string());
        let surname: String = surnames
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or("John Doe".to_string());
        let gender: String = if divided_by_2 {
            String::from("female")
        } else {
            String::from("male")
        };
        let oiv_id = rand::thread_rng().gen_range(1..=3).to_string();
        let oiv_name = if oiv_id == "3" {
            String::from("Ультрамарины")
        } else if oiv_id == "2" {
            String::from("Космические Волки")
        } else {
            String::from("Дети Императора")
        };
        let mut statuses: Vec<Status> = Vec::new();
        if divided_by_4 {
            if i % 8 == 0 {
                statuses.push(Status::new("vacation"));
            } else if i % 16 == 0 {
                statuses.push(Status::new("sickLeave"));
            } else if i % 32 == 0 {
                statuses.push(Status::new("birthday"));
            } else if i % 64 == 0 {
                statuses.push(Status::new("maternityLeave"));
            }
        }
        let n = rand::thread_rng().gen_range(1..=20);

        let photo_url: String = if divided_by_3 {
            format!("{0}/w40k/{1}.jpg", base_url, n)
        } else {
            String::default()
        };

        let occupation = get_occupation()
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or("Unemployed".to_string());

        let product: StaffInfo = get_products()
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or(StaffInfo::new("Charity"));

        let organisation: StaffInfo = get_organisations()
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or(StaffInfo::new("DIT"));

        let subdivision: StaffInfo = get_divisions()
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or(StaffInfo::new("MOBILE TEAM"));

        let employment_type = if divided_by_4 {
            String::from("secondary")
        } else {
            String::from("main")
        };

        let member = StaffMember::new(
            name,
            surname,
            gender,
            photo_url,
            StaffInfo::new(occupation),
            StaffInfo {
                id: oiv_id,
                name: oiv_name,
            },
            product,
            organisation,
            subdivision,
            statuses,
            employment_type,
        );
        members.push(member);
    }

    let emp_childs_count: u32 = members
        .iter()
        .filter(|p| p.oiv.as_ref().is_some_and(|o| o.id == "1"))
        .count() as u32;

    let wolves_count: u32 = members
        .iter()
        .filter(|p| p.oiv.as_ref().is_some_and(|o| o.id == "1"))
        .count() as u32;

    let ultramarines_count = (members.len() as u32) - emp_childs_count - wolves_count;

    let emp_childs = Oiv {
        id: 1,
        name: "Дети Императора".to_string(),
        count: emp_childs_count,
        minister: Minister {
            id: 101,
            first_name: "Фулгрим".to_string(),
            middle_name: "".to_string(),
            last_name: "".to_string(),
            full_name: "Фулгрим".to_string(),
            image_u_r_l: Option::Some("/w40k/fg.jpg".to_string()),
            description: "Примарх ДИТ".to_string(),
        },
    };

    let space_wolves = Oiv {
        id: 2,
        name: "Космические Волки".to_string(),
        count: wolves_count,
        minister: Minister {
            id: 102,
            first_name: "Леман".to_string(),
            middle_name: "".to_string(),
            last_name: "Русс".to_string(),
            full_name: "Леман Русс".to_string(),
            image_u_r_l: Option::Some("/w40k/lr.jpg".to_string()),
            description: "Примарх ГИН".to_string(),
        },
    };

    let ultramarines = Oiv {
        id: 3,
        name: "Ультрамарины".to_string(),
        count: ultramarines_count,
        minister: Minister {
            id: 103,
            first_name: "Робаут".to_string(),
            middle_name: "".to_string(),
            last_name: "Жиллиман".to_string(),
            full_name: "Робаут Жиллиман".to_string(),
            image_u_r_l: Option::Some("/w40k/rg.jpg".to_string()),
            description: "Примарх МосГорТРАНСов".to_string(),
        },
    };

    oivs.push(emp_childs);
    oivs.push(space_wolves);
    oivs.push(ultramarines);

    DataSet {
        members: members,
        oivs: oivs,
    }
}
