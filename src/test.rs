#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::filter::{get_filter, MockValueProcessor, ValueProvider};
    use crate::model::playlist::{PlaylistItem, PlaylistItemHeader, PlaylistItemType, XtreamCluster};

    fn create_mock_pli(name: &str, group: &str) -> PlaylistItem {
        PlaylistItem {
            header: RefCell::new(PlaylistItemHeader {
                uuid: Rc::new("".to_string()),
                stream_id: Rc::new("".to_string()),
                id: Rc::new("".to_string()),
                name: Rc::new(name.to_string()),
                chno: Rc::new("".to_string()),
                logo: Rc::new("".to_string()),
                logo_small: Rc::new("".to_string()),
                group: Rc::new(group.to_string()),
                title: Rc::new("".to_string()),
                parent_code: Rc::new("".to_string()),
                audio_track: Rc::new("".to_string()),
                time_shift: Rc::new("".to_string()),
                rec: Rc::new("".to_string()),
                url: Rc::new("".to_string()),
                epg_channel_id: None,
                xtream_cluster: XtreamCluster::Live,
                additional_properties: None,
                item_type: PlaylistItemType::Live,
                series_fetched: false,
                category_id: 0,
                input_id: 0,
            })
        }
    }

    #[test]
    fn test_filter_1() {
        let flt1 = r#"(Group ~ "A" OR Group ~ "B") AND (Name ~ "C" OR Name ~ "D" OR Name ~ "E") OR (NOT (Title ~ "F") AND NOT Title ~ "K")"#;
        match get_filter(flt1, None) {
            Ok(filter) => {
                assert_eq!(format!("{filter}"), flt1);
            },
            Err(e) => {
                panic!("{}", e)
            }
        }
    }
    #[test]
    fn test_filter_2() {
        let flt2 = r#"Group ~ "d" AND ((Name ~ "e" AND NOT ((Name ~ "c" OR Name ~ "f"))) OR (Name ~ "a" OR Name ~ "b"))"#;
        match get_filter(flt2, None) {
            Ok(filter) => {
                assert_eq!(format!("{filter}"), flt2);
            },
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    #[test]
    fn test_filter_3() {
        let flt = r#"Group ~ "d" AND ((Name ~ "e" AND NOT ((Name ~ "c" OR Name ~ "f"))) OR (Name ~ "a" OR Name ~ "b")) AND (Type = vod)"#;
        match get_filter(flt, None) {
            Ok(filter) => {
                assert_eq!(format!("{filter}"), flt);
            },
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    #[test]
    fn test_filter_4() {
        let flt = r#"NOT (Name ~ ".*24/7.*" AND Group ~ "^US.*")"#;
        match get_filter(flt, None) {
            Ok(filter) => {
                assert_eq!(format!("{filter}"), flt);
                let channels = vec![
                    create_mock_pli("24/7: Cars", "FR Channels"),
                    create_mock_pli("24/7: Cars", "US Channels"),
                    create_mock_pli("Entertainment", "US Channels"),
                ];
                let mut processor = MockValueProcessor {};
                let filtered: Vec<&PlaylistItem> = channels.iter().filter(|&chan| {
                    let provider = ValueProvider { pli: RefCell::new(chan) };
                    filter.filter(&provider, &mut processor)
                }).collect();
                assert_eq!(filtered.len(), 2);
                assert_eq!(filtered.iter().any(|&chan| {
                    let group = chan.header.borrow().group.to_string();
                    let name = chan.header.borrow().name.to_string();
                    name.eq("24/7: Cars") && group.eq("FR Channels")
                }), true);
                assert_eq!(filtered.iter().any(|&chan| {
                    let group = chan.header.borrow().group.to_string();
                    let name = chan.header.borrow().name.to_string();
                    name.eq("Entertainment") && group.eq("US Channels")
                }), true);
                assert_eq!(filtered.iter().any(|&chan| {
                    let group = chan.header.borrow().group.to_string();
                    let name = chan.header.borrow().name.to_string();
                    name.eq("24/7: Cars") && group.eq("US Channels")
                }), false);
            },
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    #[test]
    fn test_filter_5() {
        let flt = r#"NOT (Name ~ "NC" OR Group ~ "GA") AND (Name ~ "NA" AND Group ~ "GA") OR (Name ~ "NB" AND Group ~ "GB")"#;
        match get_filter(flt, None) {
            Ok(filter) => {
                assert_eq!(format!("{filter}"), flt);
                let channels = vec![
                    create_mock_pli("NA", "GA"),
                    create_mock_pli("NB", "GB"),
                    create_mock_pli("NA", "GB"),
                    create_mock_pli("NB", "GA"),
                    create_mock_pli("NC", "GA"),
                    create_mock_pli("NA", "GC"),
                ];
                let mut processor = MockValueProcessor {};
                let filtered: Vec<&PlaylistItem> = channels.iter().filter(|&chan| {
                    let provider = ValueProvider { pli: RefCell::new(chan) };
                    filter.filter(&provider, &mut processor)
                }).collect();
                assert_eq!(filtered.len(), 1);
            },
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

}