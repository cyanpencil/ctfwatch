//use chrono::{DateTime, Duration, FixedOffset, Local, Offset, Utc};
//use lazy_static::lazy_static;
//use regex::Regex;
//use serde_derive::Deserialize;
//use slack_hook::{Attachment, AttachmentBuilder};

//const BASE_URL: &str = "https://ctftime.org";

//#[derive(Deserialize, Debug, Eq, PartialEq)]
//pub struct Config {
    //pub webhook_url: String,
    //pub days_into_future: i64,
    //pub color_jeopardy: String,
    //pub color_attack_defense: String,
    //pub bot_icon: Option<String>,
    //pub always_show_ctfs: Vec<usize>,
    //pub mattermost_channel: Option<String>,
//}

//#[test]
//fn test_load_config() {
    //dotenv::dotenv().expect("Failed to read .env file");
    //let config = envy::from_env::<Config>().expect("Couldn't read config");
    //let expected = Config {
        //webhook_url: "".to_string(),
        //days_into_future: 21,
        //color_jeopardy: "#0099e1".to_string(),
        //color_attack_defense: "#da5422".to_string(),
        //bot_icon: Some("https://ctftime.org/static/images/ctftime-logo-avatar.png".to_string()),
        //always_show_ctfs: vec![6, 7, 24, 412],
        //mattermost_channel: None,
    //};
    //assert_eq!(config, expected)
//}

//lazy_static! {
    //pub static ref CONFIG: Config = {
        //dotenv::dotenv().expect("Failed to read .env file");
        //envy::from_env::<Config>().expect("Couldn't read config")
    //};
    //pub static ref RE_RATING_WEIGHT: Regex =
        //Regex::new(r"Rating weight:\s*(?P<weight>\d+)").unwrap();
//}

//#[derive(Debug, Deserialize)]
//pub struct CtfEvent {
    ///// Event title, this is specific to one event, e.g. "FAUST CTF 2017"
    //title: String,
    ///// Link to CTF time page of event
    //ctftime_url: String,
    ///// Event id
    //id: usize,
    ///// Start time
    //#[serde(rename = "start")]
    //start_date: DateTime<FixedOffset>,
    ///// End time
    //#[serde(rename = "finish")]
    //finish_date: DateTime<FixedOffset>,
    ///// URL of logo
    //#[serde(rename = "logo", with = "::serde_with::rust::string_empty_as_none")]
    //logo_url: Option<String>,
    ///// Link to the event page
    //#[serde(with = "::serde_with::rust::string_empty_as_none")]
    //url: Option<String>,
    ///// format style of CTF, most common Jeopardy or AttackDefense
    //format: CtfFormat,
    ///// Determines if the public is allowed to vote for the final weight
    //public_votable: bool,
    ///// The weight of the event
    //weight: f32,
    ///// A link to the live feed of the event
    //#[serde(with = "::serde_with::rust::string_empty_as_none")]
    //live_feed: Option<String>,
    ///// Access restrictions for this event
    //restrictions: CtfRestrictions,
    ///// Location of an onsite CTF. Should be set if `onsite` is true.
    //#[serde(with = "::serde_with::rust::string_empty_as_none")]
    //location: Option<String>,
    ///// Specifies that the event is at a specific location, `location` should be set in this case
    //onsite: bool,
    ///// List of all the organizer teams
    //organizers: Vec<CtfTeam>,
    ///// ID of the general event
    //ctf_id: usize,
    ///// Number of teams who want to participate
    //participants: usize,
//}

//fn format_duration(d: &Duration) -> String {
    //let mut d = *d;
    //let mut tmp = Vec::with_capacity(4);
    //if d.num_hours() > 48 {
        //tmp.push(format!("{} days", d.num_days()));
        //d = d + Duration::days(-d.num_days());
    //}
    //if d.num_hours() > 0 {
        //tmp.push(format!("{} hours", d.num_hours()));
        //d = d + Duration::hours(-d.num_hours());
    //}
    //if d.num_minutes() > 0 {
        //tmp.push(format!("{} minutes", d.num_minutes()));
        //d = d + Duration::minutes(-d.num_minutes());
    //}
    //if d.num_seconds() > 0 {
        //tmp.push(format!("{} seconds", d.num_seconds()));
    //}
    //tmp.join(" ")
//}

//impl CtfEvent {
    //pub fn to_slack(&self) -> Attachment {
        //let duration = format_duration(&self.finish_date.signed_duration_since(self.start_date));
        //let title = format!("{} — {}", self.title, self.format.to_string());
        //let organizers = ((&self.organizers)
            //.iter()
            //.map(CtfTeam::to_markdown_link)
            //.collect::<Vec<_>>())
        //.join(", ");
        //let url = self.url.clone().unwrap_or_else(|| self.ctftime_url.clone());

        //let mut text = format!(
            //r#"**Date:** {} for {}
//{}**Organizers:** {}
//[{url:}]({url:})

//"#,
            //self.start_date.with_timezone(&Local).format("%A, %F %R"),
            //duration,
            //if let Some(rating) = self.rating_weight() {
                //format!("**Rating**: {}\n", rating)
            //} else {
                //"".to_string()
            //},
            //organizers,
            //url = url
        //);

        //if self.onsite {
            //if let Some(ref location) = self.location {
                //text += &format!("**Location:** {}\n", location);
            //}
        //}
        //if self.restrictions == CtfRestrictions::Prequalified {
            //text += "Prequalified teams only\n"
        //}

        //let fallback = format!(
            //"{}\nDate: {} for {}\n{}",
            //title,
            //self.start_date.with_timezone(&Local).naive_local(),
            //duration,
            //url
        //);

        //let mut builder = AttachmentBuilder::new(fallback)
            //.title(title)
            //.title_link(&*self.ctftime_url)
            //.text(text.trim().to_string())
            //.color(if self.format == CtfFormat::AttackDefense {
                //CONFIG.color_attack_defense.clone()
            //} else {
                //CONFIG.color_jeopardy.clone()
            //});

        //if let Some(ref url) = self.logo_url {
            //builder = builder.thumb_url(url.as_ref());
        //}

        //builder.build().unwrap()
    //}

    ///// Determines if this event should be printed
    /////
    ///// Reasons to exclude it are it is too far in the future or it is not availble online.
    //pub fn should_print_event(&self) -> bool {
        //if CONFIG.always_show_ctfs.contains(&self.ctf_id) {
            //return true;
        //}

        //if self.restrictions != CtfRestrictions::Open
            //&& self.restrictions != CtfRestrictions::Academic
        //{
            //return false;
        //}
        //let days_into_future = (self
            //.start_date
            //.signed_duration_since(Utc::now().with_timezone(&Utc.fix())))
        //.num_days();
        //!self.onsite && days_into_future <= CONFIG.days_into_future
    //}

    //pub fn rating_weight(&self) -> Option<u32> {
        //Some(self.weight.floor() as u32)
    //}
//}

//#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
//pub enum CtfRestrictions {
    //Open,
    //Prequalified,
    //Academic,
    //Invited,
    //#[serde(rename = "High-school")]
    //HighSchool,
//}

///// What type of CTF, e.g. `AttackDefense`
//#[derive(Clone, Copy, Debug, Eq, PartialEq)]
//pub enum CtfFormat {
    //Jeopardy,
    //AttackDefense,
    //HackQuest,
    //Unknown,
//}

//impl<'de> serde::de::Deserialize<'de> for CtfFormat {
    //fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    //where
        //D: serde::Deserializer<'de>,
    //{
        //use serde::de::*;
        //struct CtfFormatVisitor;

        //impl<'de> serde::de::Visitor<'de> for CtfFormatVisitor {
            //type Value = CtfFormat;

            //fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                //formatter.write_str("one of `Jeopardy`, `Attack-Defense`, `Hack quest`, ``")
            //}

            //fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            //where
                //E: serde::de::Error,
            //{
                //use crate::CtfFormat::*;
                //match value {
                    //"Jeopardy" => Ok(Jeopardy),
                    //"Attack-Defense" => Ok(AttackDefense),
                    //"Hack quest" => Ok(HackQuest),
                    //"" => Ok(Unknown),
                    //_ => Err(Error::invalid_value(Unexpected::Str(value), &self)),
                //}
            //}
        //}

        //deserializer.deserialize_str(CtfFormatVisitor)
    //}
//}

//impl CtfFormat {
    //fn to_string(&self) -> &str {
        //match *self {
            //CtfFormat::Jeopardy => "Jeopardy",
            //CtfFormat::AttackDefense => "Attack-Defense",
            //CtfFormat::HackQuest => "Hack-Quest",
            //CtfFormat::Unknown => "Unknown",
        //}
    //}
//}

///// Represent a team within ctftime
//#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
//pub struct CtfTeam {
    //id: usize,
    //name: String,
//}

//impl CtfTeam {
    //pub fn to_markdown_link(&self) -> String {
        //format!("[{}]({}/team/{})", self.name, BASE_URL, self.id)
    //}
//}

//#[allow(clippy::float_cmp)]
//#[test]
//fn test_deserialize_ctf_event() {
    //use std::fs::File;
    //let json = File::open("./tests/ctfs.json").unwrap();

    //let res: Vec<CtfEvent> = serde_json::from_reader(json).unwrap();
    //assert_eq!(res.len(), 442);

    //let event = &res[440];
    //assert_eq!(event.onsite, false);
    //assert_eq!(event.weight, 25.0);
    //assert_eq!(event.rating_weight(), Some(25));
    //assert_eq!(event.title, "RHme3 - Qualifiers");
    //assert_eq!(event.url, Some("https://rhme.riscure.com/3/".to_string()));
    //assert_eq!(event.restrictions, CtfRestrictions::Open);
    //assert_eq!(event.format, CtfFormat::Jeopardy);
    //assert_eq!(event.participants, 0);
    //assert_eq!(event.ctftime_url, "https://ctftime.org/event/501/");
    //assert_eq!(event.location, None);
    //assert_eq!(
        //event.live_feed,
        //Some("https://ctftime.org/live/501/".to_string())
    //);
    //assert_eq!(event.public_votable, true);
    //assert_eq!(
        //event.logo_url,
        //Some("https://ctftime.org//media/events/RHME3_logo_Box.png".to_string())
    //);
    //assert_eq!(event.id, 501);
    //assert_eq!(event.ctf_id, 209);

    //let event = &res[441];
    //assert_eq!(event.onsite, true);
    //assert_eq!(event.weight, 0.0);
    //assert_eq!(event.rating_weight(), Some(0));
    //assert_eq!(
        //event.title,
        //"Hardwear.io - Hardware Security Conference and Training"
    //);
    //assert_eq!(event.url, Some("http://hardwear.io/".to_string()));
    //assert_eq!(event.restrictions, CtfRestrictions::Open);
    //assert_eq!(event.format, CtfFormat::AttackDefense);
    //assert_eq!(event.participants, 4);
    //assert_eq!(event.ctftime_url, "https://ctftime.org/event/514/");
    //assert_eq!(
        //event.location,
        //Some("NH Hotel, The Hague, Netherlands".to_string())
    //);
    //assert_eq!(event.live_feed, None);
    //assert_eq!(event.public_votable, false);
    //assert_eq!(
        //event.logo_url,
        //Some("https://ctftime.org//media/events/hardwear_500Dpi_-_Copy.jpg".to_string())
    //);
    //assert_eq!(event.id, 514);
    //assert_eq!(event.ctf_id, 216);
//}

//#[test]
//fn test_deserialize_ctf_event_rating_weight() {
    //use std::fs::File;
    //let json = File::open("./tests/ctfs-1.json").unwrap();

    //let res: Vec<CtfEvent> = serde_json::from_reader(json).unwrap();
    //assert_eq!(res.len(), 1);

    //// Test first
    //let event = &res[0];
    //assert_eq!(event.ctftime_url, "https://ctftime.org/event/724/");
    //assert_eq!(event.rating_weight(), Some(24));
//}
