/// 유튜브 음악 리스트 관리
/// 
/// 특정 이름의 플레이 리스트를 생성하여
/// 해당 플레이 리스트에 음악을 추가
/// 
/// https://github.com/Byron/google-apis-rs 라이브러리 및 예제 사용
/// 
/// 리스트 조회 및 업데이트에는 oauth2 인증 토큰이 필수.
/// cli를 통해 주소를 받고 구글 계정 인증을 해야함.
/// 위 라이브러리에 구현되어있음.
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_youtube3 as youtube3;
use youtube3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use youtube3::YouTube;

pub fn example_function{
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
    // `client_secret`, among other things.
    let secret: ApplicationSecret = Default::default();
    // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                              <MemoryStorage as Default>::default(), None);
    let mut hub = YouTube::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.videos().list("part")
                .video_category_id("sit")
                .region_code("Stet")
                .page_token("sed")
                .on_behalf_of_content_owner("et")
                .my_rating("dolores")
                .max_width(38)
                .max_results(79)
                .max_height(93)
                .locale("justo")
                .id("amet.")
                .hl("erat")
                .chart("labore")
                .doit();

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn example_function_test(){
        example_function();
    }
}