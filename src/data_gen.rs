use crate::models::{Count, Head, Oiv, Organization, StaffInfo, StaffMember, Status};
use rand::Rng;
use rand::seq::SliceRandom;
use std::{fmt::format, mem, vec};

#[derive(Debug, Clone)]
pub struct DataSet {
    pub members: Vec<StaffMember>,
    pub oivs: Vec<Oiv>,
    pub organisations: Vec<StaffInfo>,
    pub products: Vec<StaffInfo>,
    pub divisions: Vec<StaffInfo>,
    pub locations: Vec<StaffInfo>,
    pub addresses: Vec<StaffInfo>,
}

fn get_organisations() -> Vec<StaffInfo> {
    vec![
        StaffInfo {
            id: 1.to_string(),
            name: format!("Адептус Астартес"),
        },
        StaffInfo {
            id: 1.to_string(),
            name: format!("Адепта Сороритас"),
        },
        StaffInfo {
            id: 1.to_string(),
            name: format!("Адептус Механикус"),
        },
        // StaffInfo {}::new("Адептус Астартес"),
        // StaffInfo::new("Адепта Сороритас"),
        // StaffInfo::new("Адептус Механикус"),
        // StaffInfo::new("Империум Человечества"),
        // StaffInfo::new("Имперская Гвардия"),
        // StaffInfo::new("Инквизиция"),
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
        // "Разоритель",
        // "Коракс",
        // "Красный",
        // "Тигурий",
        // "Вентан",
        // "Арвида",
        // "Котэс",
        // "Страйфсон",
        // "Шрайк",
        // "Аргел",
        // "Вентрис",
        // "Мортарион",
        // "Рем",
        // "Зелл",
        // "Гримальдус",
        // "Таранис",
        // "Шакал",
        // "Талос",
        // "Малхарион",
        // "Бетанкор",
        // "Рун",
        // "Кёрз",
        // "Тигурий",
        // "Вардус",
        // "Литтл",
        // "Флойд",
        // "Нокс",
        // "Драк",
        // "Кроун",
        // "Локен",
        // "Севатар",
        // "Руна",
        // "Вулф",
        // "Терор",
        // "Голдинг",
        // "Дракол",
        // "Норен",
        // "Веланид",
        // "Карзиан",
        // "Трос",
        // "Фаррелл",
        // "Виндоу",
        // "Прайм",
        // "Мерваллион",
        // "Алонсо",
        // "Сторм",
        // "Грей",
        // "Дюран",
        // "Кортез",
        // "Гордан",
        // "Варден",
        // "Нортон",
        // "Роллинс",
        // "Грейвс",
        // "Вест",
        // "Тарион",
        // "Каннингем",
        // "Черч",
        // "Барретт",
        // "Грант",
        // "Рейн",
        // "Дарк",
        // "Фрост",
        // "Кэрролл",
        // "Макгрегор",
        // "Кросс",
        // "Фергюсон",
        // "Холл",
        // "Элвин",
        // "Фарроу",
        // "Блэк",
        // "Олден",
        // "Серил",
        // "Драммонд",
        // "Брукс",
        // "Сиверс",
        // "Каллис",
        // "Грейв",
        // "Хант",
        // "Флеминг",
        // "Брент",
        // "Уоллес",
        // "Картер",
        // "Дрейк",
        // "Ламбер",
        // "Ренар",
        // "Кинкейд",
        // "Макконнелл",
        // "Гилберт",
        // "Сандерс",
        // "Перри",
        // "Киллер",
        // "Рандольф",
        // "Фаулер",
        // "Харрисон",
        // "Прайс",
        // "Томлинсон",
        // "Эймс",
        // "Клейтон",
        // "Лафлин",
        // "Харпер",
        // "Кармайкл",
        // "Уитман",
        // "Джарретт",
        // "Нэш",
        // "Бишоп",
        // "Макаллистер",
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

pub fn get_heads(base_url: &str) -> Vec<Head> {
    vec![
        Head {
            id: 101,
            first_name: "Фулгрим".to_string(),
            middle_name: "".to_string(),
            last_name: "".to_string(),
            position: "Примарх ДИТ".to_string(),
            image_url: Option::Some(format!("{0}/public/w40k/{1}.jpg", base_url, "fg")),
        },
        Head {
            id: 102,
            first_name: "Леман".to_string(),
            middle_name: "".to_string(),
            last_name: "Русс".to_string(),
            position: "Примарх ГИН".to_string(),
            image_url: Option::Some(format!("{0}/public/w40k/{1}.jpg", base_url, "lr")),
        },
        Head {
            id: 103,
            first_name: "Робаут".to_string(),
            middle_name: "".to_string(),
            last_name: "Жиллиман".to_string(),
            position: "Примарх МосГорТРАНСов".to_string(),
            image_url: Option::Some(format!("{0}/public/w40k/{1}.jpg", base_url, "rg")),
        },
    ]
}

pub fn get_org(base_url: &str) -> Vec<Organization> {
    let heads = get_heads(base_url);
    vec![
        Organization {
            id: 1,
            icon_url: format!("{0}/public/w40k/5.jpg", base_url),
            name: format!("Адептус Астартес"),
            full_name: format!("Адептус Астартес"),
            head: heads[2].clone(),
        },
        Organization {
            id: 2,
            icon_url: format!("{0}/public/w40k/10.jpg", base_url),
            name: format!("Адепта Сороритас"),
            full_name: format!("Адепта Сороритас"),
            head: heads[1].clone(),
        },
        Organization {
            id: 3,
            icon_url: format!("{0}/public/w40k/15.jpg", base_url),
            name: format!("Адептус Механикус"),
            full_name: format!("Адептус Механикус"),
            head: heads[0].clone(),
        },
    ]
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
            format!("{0}/public/w40k/{1}.jpg", base_url, n)
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

    members.sort_by_key(|f| f.oiv.id.clone());
    let emp_childs_count: u32 = members.iter().filter(|p| p.oiv.id == "0").count() as u32;

    let wolves_count: u32 = members.iter().filter(|p| p.oiv.id == "1").count() as u32;

    let ultramarines_count = (members.len() as u32) - emp_childs_count - wolves_count;

    let heads = get_heads(&base_url);

    let emp_childs = Oiv {
        id: 1,
        icon_url: None,
        short_name: "Дети Императора".to_string(),
        name: "ООО КПК ИТД Дети его величества священного Императора правящего мирами".to_string(),
        count: Count {
            employees: Some(emp_childs_count),
            organizations: Some(1),
        },
        head: Some(heads[0].clone()),
        structure_type: vec![format!("staffpositions")],
    };

    let space_wolves = Oiv {
        id: 2,
        icon_url: None,
        short_name: "Космические Волки".to_string(),
        name: "ООО КПК ИТД Космические Волки и вообще я уже пожалел что выбрал вархаммер как рыбный контент".to_string(),
        count: Count {
            employees: Some(wolves_count),
            organizations: Some(1),
        },
        head: Some(heads[1].clone()),
        structure_type: vec![format!("management")],
    };

    let ultramarines = Oiv {
        id: 3,
        icon_url: None,
        short_name: "Ультрамарины".to_string(),
        name: "ООО КПК ИТД Ультрамарины ака фиолетовые парни с пушками".to_string(),
        count: Count {
            employees: Some(ultramarines_count),
            organizations: Some(2),
        },
        head: Some(heads[2].clone()),
        structure_type: vec![format!("staffpositions"), format!("management")],
    };

    oivs.push(emp_childs);
    oivs.push(space_wolves);
    oivs.push(ultramarines);

    DataSet {
        members: members,
        oivs: oivs,
        organisations: get_organisations(),
        products: get_products(),
        divisions: get_divisions(),
        locations: get_locations(),
        addresses: get_addresses(),
    }
}
