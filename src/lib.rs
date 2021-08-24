use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// RFC Group type
#[derive(Serialize, Deserialize, Debug)]
pub struct RfcGroup {
    /// Group name
    pub name: String,
    /// Group type
    #[serde(rename = "type")]
    pub kind: String,
    /// Possible group acronym
    pub acronym: String,
}

/// RFC Author
#[derive(Serialize, Deserialize, Debug)]
pub struct RfcAuthor {
    /// Author name
    pub name: String,
    /// Author email
    pub email: String,
    /// Author affiliation
    pub affiliation: String,
}

/// RFC Revision
#[derive(Serialize, Deserialize, Debug)]
pub struct RfcRevision {
    /// Revision name
    pub name: String,
    /// Revision ID
    pub rev: String,
    /// Publish timestamp
    pub published: DateTime<Utc>,
    /// Url fragment
    url: String,
}

impl RfcRevision {
    /// Get the revision URL
    pub fn url(&self) -> String {
        format!("https://datatracker.ietf.org{}", self.url)
    }
}

/// Defines an RFC
#[derive(Deserialize, Serialize, Debug)]
pub struct Rfc {
    /// Name
    pub name: String,
    /// Revision
    pub rev: String,
    /// Number of pages in the document
    pub pages: u32,
    /// Time of last update
    pub time: String,
    /// Group this RFC belongs to
    pub group: RfcGroup,
    /// RFC expiry
    pub expires: Option<String>,
    /// RFC title
    pub title: String,
    /// RFC abstract
    #[serde(rename = "abstract")]
    pub description: String,
    /// Any aliases to this RFC
    pub aliases: Vec<String>,
    /// State of the RFC
    pub state: String,
    /// Intended std level
    pub intended_std_level: Option<String>,
    /// Current std level
    pub std_level: String,
    /// All RFC authors
    pub authors: Vec<RfcAuthor>,
    /// RFC shepherd
    pub shepherd: Option<String>,
    /// RFC ad
    pub ad: Option<String>,
    /// IESG state
    pub iesg_state: String,
    /// RFCEditor state
    pub rfceditor_state: Option<String>,
    /// State of IANA review
    pub iana_review_state: Option<String>,
    /// State of IANA action
    pub iana_action_state: Option<String>,
    /// RFC stream
    pub stream: Option<String>,
}

impl Rfc {
    /// Get the URL to the text version of this RFC
    pub fn get_txt_url(&self) -> String {
        format!("https://www.rfc-editor.org/rfc/{}.txt", self.name)
    }
    /// Get the URL to the PDF version of thisRFC
    pub fn get_pdf_url(&self) -> String {
        format!("https://www.rfc-editor.org/rfc/{}.txt.pdf", self.name)
    }
    // Get the URL to the HTML version of this RFC
    pub fn get_html_url(&self) -> String {
        format!("https://www.rfc-editor.org/rfc/{}.html", self.name)
    }
}

/// Query for an RFC.
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// let rfc = rfclib::query_rfc(2549).await.unwrap();
/// assert_eq!(rfc.name, "rfc2549");
/// assert_eq!(rfc.title, "IP over Avian Carriers with Quality of Service");
/// # })
/// ```
pub async fn query_rfc(number: u32) -> Result<Rfc, reqwest::Error> {
    Ok(reqwest::get(format!(
        "https://datatracker.ietf.org/doc/rfc{}/doc.json",
        number
    ))
    .await?
    .json()
    .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_query_rfc_failure() {
        tokio_test::block_on(async {
            let rfc = query_rfc(9999999).await;
            assert!(rfc.is_err());
        })
    }
}
