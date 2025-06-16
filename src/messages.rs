use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct MessageTemplates {
    rng: StdRng,
}

impl MessageTemplates {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_entropy(),
        };
        Self { rng }
    }

    pub fn harvest_message(&mut self, yield_per_acre: u32, total: u32) -> String {
        let templates = [
            "THY HARVEST YIELDED {} BUSHELS PER ACRE, TOTAL: {} BUSHELS",
            "THE GODS SMILED UPON THY FIELDS - {} BUSHELS PER ACRE, {} BUSHELS IN TOTAL", 
            "THY LABORERS REAPED {} BUSHELS FROM EACH ACRE, GATHERING {} BUSHELS",
            "BEHOLD! EACH ACRE PRODUCED {} BUSHELS, TOTALING {} BUSHELS",
            "THE EARTH GRANTED THEE {} BUSHELS PER ACRE, {} BUSHELS NOW FILL THY STORES",
            "THY FIELDS BORE FRUIT - {} BUSHELS PER ACRE, {} BUSHELS HARVESTED",
            "FROM EACH ACRE CAME {} BUSHELS, THY GRANARIES RECEIVED {} BUSHELS",
            "THE HARVEST IS COMPLETE - {} BUSHELS PER ACRE, {} BUSHELS TOTAL",
            "THY TOILING SUBJECTS GATHERED {} BUSHELS PER ACRE, AMASSING {} BUSHELS",
            "FORTUNE FAVORED THY CROPS - {} BUSHELS PER ACRE, {} BUSHELS IN ALL",
        ];
        
        let template = templates.choose(&mut self.rng).unwrap();
        template.replacen("{}", &yield_per_acre.to_string(), 1)
            .replacen("{}", &total.to_string(), 1)
    }

    pub fn rats_message(&mut self, amount: u32) -> String {
        let templates = [
            "RATS INFERNAL DEVOURED {} BUSHELS OF THY GRAIN!",
            "A PLAGUE OF RATS CONSUMED {} BUSHELS FROM THY STORES!",
            "VERMIN MOST FOUL ATE {} BUSHELS OF GRAIN!",
            "THY GRANARIES WERE BREACHED - RATS STOLE {} BUSHELS!",
            "CURSED RODENTS MADE OFF WITH {} BUSHELS!",
            "THE RAT HORDES FEASTED UPON {} BUSHELS OF THY GRAIN!",
            "{} BUSHELS FELL PREY TO THE GNAWING PESTILENCE!",
            "ALAS! RATS DESTROYED {} BUSHELS IN THY STOREHOUSES!",
            "THE SCURRYING MENACE CLAIMED {} BUSHELS!",
            "THY GRAIN SUFFERED - {} BUSHELS LOST TO RATS!",
        ];
        
        let template = templates.choose(&mut self.rng).unwrap();
        template.replace("{}", &amount.to_string())
    }

    pub fn plague_message(&mut self) -> String {
        let templates = [
            "A GREAT PESTILENCE HATH SWEPT THY KINGDOM! HALF THY SUBJECTS PERISHED!",
            "PLAGUE MOST TERRIBLE STRUCK! HALF OF THY PEOPLE ARE DEAD!",
            "THE BLACK DEATH VISITED THY REALM! HALF THY POPULATION SUCCUMBED!",
            "DISEASE RAVAGED THY LANDS! HALF OF THY SUBJECTS HAVE DIED!",
            "A HORRIBLE SICKNESS BEFELL THY PEOPLE! HALF ARE NOW DECEASED!",
            "PESTILENCE CLAIMED HALF OF THY SUBJECTS' LIVES!",
            "THE GODS' WRATH MANIFESTED AS PLAGUE! HALF THY PEOPLE PERISHED!",
            "DEATH'S SHADOW FELL UPON THY KINGDOM! HALF ARE GONE!",
            "A VILE CONTAGION SPREAD! HALF OF THY POPULATION IS NO MORE!",
            "PLAGUE DEMONS VISITED THY REALM! HALF THY SUBJECTS DIED!",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn immigration_message(&mut self, amount: u32) -> String {
        let templates = [
            "{} SOULS CAME TO DWELL IN THY KINGDOM",
            "{} NEW SUBJECTS ARRIVED SEEKING THY PROTECTION",
            "THY FAME ATTRACTED {} NEW CITIZENS",
            "{} PEOPLE JOURNEYED TO THY PROSPEROUS REALM",
            "WORD OF THY WISDOM DREW {} NEW INHABITANTS",
            "{} SOULS SOUGHT REFUGE IN THY DOMAIN",
            "THY KINGDOM WELCOMED {} NEW ARRIVALS",
            "{} IMMIGRANTS SWELLED THY POPULATION",
            "THE PROMISE OF THY RULE BROUGHT {} NEW SUBJECTS",
            "{} PEOPLE CAME TO SERVE UNDER THY BANNER",
        ];
        
        let template = templates.choose(&mut self.rng).unwrap();
        template.replace("{}", &amount.to_string())
    }

    pub fn no_immigration_message(&mut self) -> String {
        let templates = [
            "NO SOULS DARE ENTER THY STARVING KINGDOM",
            "THY REPUTATION REPELS ALL WOULD-BE IMMIGRANTS",
            "NONE WISH TO JOIN A REALM OF STARVATION",
            "WORD OF THY FAILURES KEEPS ALL AWAY",
            "NO ONE COMES TO A LAND OF DEATH",
            "THY KINGDOM'S INFAMY PREVENTS IMMIGRATION",
            "THE STARVING MASSES DISCOURAGE NEW ARRIVALS",
            "NONE SEEK TO SHARE THY SUBJECTS' FATE",
            "THY MISMANAGEMENT IS KNOWN - NONE WILL COME",
            "A KINGDOM OF HUNGER ATTRACTS NO NEW SOULS",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn starvation_message(&mut self, amount: u32) -> String {
        let templates = [
            "{} OF THY SUBJECTS STARVED TO DEATH",
            "HUNGER CLAIMED {} SOULS IN THY KINGDOM",
            "{} PEOPLE PERISHED FROM LACK OF FOOD",
            "THY MISERLY RATIONING KILLED {} SUBJECTS",
            "FAMINE TOOK {} OF THY PEOPLE",
            "{} DIED WITH EMPTY BELLIES CURSING THY NAME",
            "STARVATION REAPED {} LIVES FROM THY REALM",
            "{} SUBJECTS SUCCUMBED TO HUNGER'S CRUEL EMBRACE",
            "THY GRANARIES FAILED {} WHO NOW LIE DEAD",
            "{} STARVED UNDER THY WATCH",
        ];
        
        let template = templates.choose(&mut self.rng).unwrap();
        template.replace("{}", &amount.to_string())
    }

    pub fn insufficient_grain_land_message(&mut self) -> String {
        let templates = [
            "THY COFFERS LACK THE GRAIN FOR SUCH PURCHASE!",
            "THOU HAST NOT ENOUGH GRAIN FOR THIS TRANSACTION!",
            "THY STORES CANNOT SUPPORT SUCH AMBITION!",
            "INSUFFICIENT GRAIN FOR THY GRAND DESIGNS!",
            "THE GRANARIES PROTEST - NOT ENOUGH GRAIN!",
            "THY REACH EXCEEDS THY GRASP - MORE GRAIN NEEDED!",
            "ALAS, THY GRAIN RESERVES FALL SHORT!",
            "SUCH PURCHASE WOULD EMPTY THY STORES!",
            "THY GRAIN SUPPLIES FORBID THIS ACTION!",
            "THOU CANST NOT AFFORD SUCH LUXURY!",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn insufficient_land_message(&mut self) -> String {
        let templates = [
            "THOU DOST NOT POSSESS SUCH VAST ESTATES!",
            "THY KINGDOM IS NOT SO LARGE!",
            "THOU HAST NOT THAT MUCH LAND TO SELL!",
            "THY HOLDINGS ARE MORE MODEST THAN THOU THINKEST!",
            "SUCH ACREAGE EXISTS NOT IN THY REALM!",
            "THY LANDS ARE INSUFFICIENT FOR THIS DEED!",
            "THOU ART NOT SO RICH IN LAND!",
            "THY DOMAIN ENCOMPASSES NOT SUCH EXPANSE!",
            "CHECK THY RECORDS - THOU HAST LESS LAND!",
            "THY KINGDOM'S BORDERS CONTAIN NOT SO MUCH!",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn insufficient_grain_seed_message(&mut self) -> String {
        let templates = [
            "THY GRANARIES HOLD NOT ENOUGH SEED!",
            "INSUFFICIENT GRAIN FOR PLANTING SUCH FIELDS!",
            "THY SEED STORES CANNOT COVER SO MUCH LAND!",
            "THOU LACKEST THE GRAIN TO PLANT SO WIDELY!",
            "NOT ENOUGH SEED REMAINS IN THY STORES!",
            "THY AMBITIOUS PLANTING EXCEEDS THY GRAIN!",
            "THE SEEDMASTERS SAY: NOT ENOUGH GRAIN!",
            "THY GRANARIES CANNOT PROVIDE SUCH SEED!",
            "MORE GRAIN IS NEEDED FOR SUCH PLANTING!",
            "THY STORES LACK SUFFICIENT PLANTING GRAIN!",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn insufficient_land_planting_message(&mut self) -> String {
        let templates = [
            "THY KINGDOM ENCOMPASSES NOT SUCH ACREAGE!",
            "THOU CANST NOT PLANT MORE THAN THOU OWNEST!",
            "THY LANDS ARE NOT SO EXTENSIVE!",
            "SUCH FIELDS EXIST NOT IN THY DOMAIN!",
            "THY HOLDINGS LIMIT THY PLANTING!",
            "THOU HAST NOT ACQUIRED SUCH VAST LANDS!",
            "THY REALM IS SMALLER THAN THY AMBITIONS!",
            "CHECK THY BORDERS - NOT ENOUGH LAND!",
            "THY KINGDOM CANNOT PLANT PHANTOM FIELDS!",
            "THE LAND THOU SEEKEST TO PLANT IS NOT THINE!",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn insufficient_workers_message(&mut self) -> String {
        let templates = [
            "TOO FEW SUBJECTS REMAIN TO TILL SUCH FIELDS!",
            "THY PEOPLE CANNOT WORK SO MUCH LAND!",
            "NOT ENOUGH WORKERS FOR THY GRAND PLANS!",
            "THY SUBJECTS ARE TOO FEW FOR SUCH LABOR!",
            "EACH WORKER CAN TILL BUT TEN ACRES!",
            "THY POPULATION LIMITS THY PLANTING!",
            "MORE WORKERS NEEDED FOR SUCH AMBITION!",
            "THY PEOPLE PROTEST - TOO MUCH LAND TO WORK!",
            "INSUFFICIENT HANDS FOR SO MANY FIELDS!",
            "THY WORKFORCE CANNOT MANAGE SUCH ACREAGE!",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }

    pub fn insufficient_grain_feeding_message(&mut self) -> String {
        let templates = [
            "THY STORES CONTAIN NOT SUCH ABUNDANCE!",
            "THOU HAST NOT THAT MUCH GRAIN TO DISTRIBUTE!",
            "THY GRANARIES HOLD LESS THAN THOU BELIEVEST!",
            "INSUFFICIENT GRAIN FOR SUCH GENEROSITY!",
            "CHECK THY STORES - NOT ENOUGH GRAIN!",
            "THY GRAIN RESERVES FALL SHORT OF THY PROMISE!",
            "THE GRANARIES CANNOT PROVIDE SO MUCH!",
            "THOU ART TOO GENEROUS - NOT ENOUGH GRAIN!",
            "THY STORES PROTEST SUCH LAVISH FEEDING!",
            "ALAS, THY GRAIN IS INSUFFICIENT!",
        ];
        
        templates.choose(&mut self.rng).unwrap().to_string()
    }
}