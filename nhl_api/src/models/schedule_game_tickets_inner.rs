/*
 * NHL API
 *
 * Documenting the publicly accessible portions of the NHL API.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScheduleGameTicketsInner {
    #[serde(rename = "ticketType", skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<TicketType>,
    #[serde(rename = "ticketLink", skip_serializing_if = "Option::is_none")]
    pub ticket_link: Option<String>,
}

impl ScheduleGameTicketsInner {
    pub fn new() -> ScheduleGameTicketsInner {
        ScheduleGameTicketsInner {
            ticket_type: None,
            ticket_link: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TicketType {
    #[serde(rename = "buysell")]
    Buysell,
    #[serde(rename = "club buysell")]
    ClubBuysell,
    #[serde(rename = "club mobile")]
    ClubMobile,
    #[serde(rename = "club mobile buysell")]
    ClubMobileBuysell,
    #[serde(rename = "club ticket")]
    ClubTicket,
    #[serde(rename = "mobile app ticket")]
    MobileAppTicket,
    #[serde(rename = "mobile buysell")]
    MobileBuysell,
    #[serde(rename = "mobile ticket")]
    MobileTicket,
    #[serde(rename = "tablet app ticket")]
    TabletAppTicket,
    #[serde(rename = "ticket")]
    Ticket,
}

impl Default for TicketType {
    fn default() -> TicketType {
        Self::Buysell
    }
}

