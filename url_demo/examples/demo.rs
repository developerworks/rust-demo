use lazy_static::lazy_static;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref HALL_SERVICE_MAPS: Arc<Mutex<HashMap<String, HashMap<String, ServiceInfo>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Debug)]
struct ServiceInfo {
    loads: i32,
    people: i32,
    t: u128,
    #[allow(unused)]
    d: i32,
}

#[derive(Clone)]
struct Request {
    url: String,
    env: String,
    loads: i32,
    people: i32,
}

fn sync_hall_service(req: Request) {
    let mut hall_maps = HALL_SERVICE_MAPS.lock().unwrap();

    let env = &req.env;
    let url = &req.url;
    let loads = req.loads;
    let people = req.people;

    // let env_map = hall_maps.entry(env.clone()).or_insert_with(HashMap::new);
    //
    // let service_info = env_map.entry(url.clone()).or_insert_with(|| ServiceInfo {
    //     loads: 0,
    //     people: 0,
    //     t: 0,
    //     d: 0,
    // });
    //
    // service_info.loads = loads;
    // service_info.people = people;
    // service_info.t = 0;

    match hall_maps.entry(env.clone()) {
        Occupied(mut env_entry) => {
            let map = env_entry.get_mut();

            let r = map.get_mut(&url.clone());
            if let Some(si) = r {
                si.loads = loads;
                si.people = people;
                si.t = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
            } else {
                map.insert(
                    url.clone(),
                    ServiceInfo {
                        loads,
                        people,
                        t: SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                        d: 0,
                    },
                );
            }
        }
        Vacant(_entry) => {
            println!("env_map is vacant");
            let mut url_map: HashMap<String, ServiceInfo> = HashMap::new();
            url_map.insert(
                url.clone(),
                ServiceInfo {
                    loads,
                    people,
                    t: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis(),
                    d: 0,
                },
            );

            println!("url_map is {:?}", url_map);
            hall_maps.insert(env.clone(), url_map);
            println!("hall_maps is {:?}", hall_maps);
        }
    }
}


fn main() {
    let req = Request {
        url: "example.com".to_string(),
        env: "dev".to_string(),
        loads: 10,
        people: 5,
    };
    sync_hall_service(req);
    println!("{:#?}", *HALL_SERVICE_MAPS.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    // 空值
    fn test_sync_hall_service_0() {
        let req = Request {
            url: "example.com".to_string(),
            env: "dev".to_string(),
            loads: 10,
            people: 5,
        };
        sync_hall_service(req.clone());
        let hall_maps = HALL_SERVICE_MAPS.lock().unwrap();

        assert!(hall_maps.contains_key(&req.env));
        assert!(hall_maps.get(&req.env).unwrap().contains_key(&req.url));
        let service_info = hall_maps.get(&req.env).unwrap().get(&req.url).unwrap();
        assert_eq!(service_info.loads, req.loads);
        assert_eq!(service_info.people, req.people);

        drop(hall_maps);
    }

    // 预置值分支覆盖
    fn test_sync_hall_service_1() {
        env_logger::init();

        let req = Request {
            url: "example.com".to_string(),
            env: "dev".to_string(),
            loads: 10,
            people: 5
        };

        let mut hall_maps = HALL_SERVICE_MAPS.lock().unwrap();
        let mut url_map = HashMap::new();
        url_map.insert(
            req.url.clone(),
            ServiceInfo {
                loads: req.loads,
                people: req.people,
                t: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                d: 0,
            },
        );
        hall_maps.insert(req.env.clone(), url_map);

        assert!(hall_maps.contains_key(&req.env));
        assert!(hall_maps.get(&req.env).unwrap().contains_key(&req.url));
        let service_info = hall_maps.get(&req.env).unwrap().get(&req.url).unwrap();
        assert_eq!(service_info.loads, req.loads);
        assert_eq!(service_info.people, req.people);

        drop(hall_maps);

        sync_hall_service(req.clone());
    }

    // 预置值分支覆盖
    fn test_sync_hall_service_2() {
        let req = Request {
            url: "example_not_exists.com".to_string(),
            env: "dev".to_string(),
            loads: 10,
            people: 5
        };

        let mut hall_maps = HALL_SERVICE_MAPS.lock().unwrap();
        let url_map = HashMap::new();
        hall_maps.insert(req.env.clone(), url_map);
        // url_map.insert(
        //     req.url.clone(),
        //     None
        // );
        // hall_maps.insert(req.env.clone(), url_map);

        drop(hall_maps);

        sync_hall_service(req.clone());

        // assert!(hall_maps.contains_key(&req.env));
        // assert!(hall_maps.get(&req.env).unwrap().contains_key(&req.url));
        // let service_info = hall_maps.get(&req.env).unwrap().get(&req.url).unwrap();
        // assert_eq!(service_info.loads, req.loads);
        // assert_eq!(service_info.people, req.people);
    }

    #[test]
    fn test_demo() {
        test_sync_hall_service_0();
        test_sync_hall_service_1();
        test_sync_hall_service_2();
    }
}
