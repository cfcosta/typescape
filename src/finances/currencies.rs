pub trait Currency {
    fn ticker(&self) -> &'static str;
    fn name(&self) -> &'static str;
}

macro_rules! currency {
    ($t:tt,$name:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "testing", derive(arbitrary::Arbitrary))]
        pub struct $t;

        impl Currency for $t {
            fn ticker(&self) -> &'static str {
                stringify!($t)
            }

            fn name(&self) -> &'static str {
                $name
            }
        }
    };
}

currency!(AED, "United Arab Emirates (The)");
currency!(AFN, "Afghanistan");
currency!(ALL, "Albania");
currency!(AMD, "Armenia");
currency!(ANG, "Curaçao");
currency!(AOA, "Angola");
currency!(ARS, "Argentina");
currency!(AUD, "Australia");
currency!(AWG, "Aruba");
currency!(AZN, "Azerbaijan");
currency!(BAM, "Bosnia And Herzegovina");
currency!(BBD, "Barbados");
currency!(BDT, "Bangladesh");
currency!(BGN, "Bulgaria");
currency!(BHD, "Bahrain");
currency!(BIF, "Burundi");
currency!(BMD, "Bermuda");
currency!(BND, "Brunei Darussalam");
currency!(BOB, "Bolivia (Plurinational State Of)");
currency!(BOV, "Bolivia (Plurinational State Of)");
currency!(BRL, "Brazil");
currency!(BSD, "Bahamas (The)");
currency!(BTN, "Bhutan");
currency!(BWP, "Botswana");
currency!(BYN, "Belarus");
currency!(BZD, "Belize");
currency!(CAD, "Canada");
currency!(CDF, "Congo (The Democratic Republic Of The)");
currency!(CHE, "Switzerland");
currency!(CHF, "Switzerland");
currency!(CHW, "Switzerland");
currency!(CLF, "Chile");
currency!(CLP, "Chile");
currency!(CNY, "China");
currency!(COP, "Colombia");
currency!(COU, "Colombia");
currency!(CRC, "Costa Rica");
currency!(CUC, "Cuba");
currency!(CUP, "Cuba");
currency!(CVE, "Cabo Verde");
currency!(CZK, "Czechia");
currency!(DJF, "Djibouti");
currency!(DKK, "Denmark");
currency!(DOP, "Dominican Republic (The)");
currency!(DZD, "Algeria");
currency!(EGP, "Egypt");
currency!(ERN, "Eritrea");
currency!(ETB, "Ethiopia");
currency!(EUR, "European Union");
currency!(FJD, "Fiji");
currency!(FKP, "Falkland Islands (The Malvinas)");
currency!(
    GBP,
    "United Kingdom Of Great Britain And Northern Ireland (The)"
);
currency!(GEL, "Georgia");
currency!(GHS, "Ghana");
currency!(GIP, "Gibraltar");
currency!(GMD, "Gambia (The)");
currency!(GNF, "Guinea");
currency!(GTQ, "Guatemala");
currency!(GYD, "Guyana");
currency!(HKD, "Hong Kong");
currency!(HNL, "Honduras");
currency!(HRK, "Croatia");
currency!(HTG, "Haiti");
currency!(HUF, "Hungary");
currency!(IDR, "Indonesia");
currency!(ILS, "Israel");
currency!(INR, "India");
currency!(IQD, "Iraq");
currency!(IRR, "Iran (Islamic Republic Of)");
currency!(ISK, "Iceland");
currency!(JMD, "Jamaica");
currency!(JOD, "Jordan");
currency!(JPY, "Japan");
currency!(KES, "Kenya");
currency!(KGS, "Kyrgyzstan");
currency!(KHR, "Cambodia");
currency!(KMF, "Comoros (The)");
currency!(KPW, "Korea (The Democratic People'S Republic Of)");
currency!(KRW, "Korea (The Republic Of)");
currency!(KWD, "Kuwait");
currency!(KYD, "Cayman Islands (The)");
currency!(KZT, "Kazakhstan");
currency!(LAK, "Lao People'S Democratic Republic (The)");
currency!(LBP, "Lebanon");
currency!(LKR, "Sri Lanka");
currency!(LRD, "Liberia");
currency!(LSL, "Lesotho");
currency!(LYD, "Libya");
currency!(MAD, "Morocco");
currency!(MDL, "Moldova (The Republic Of)");
currency!(MGA, "Madagascar");
currency!(MKD, "North Macedonia");
currency!(MMK, "Myanmar");
currency!(MNT, "Mongolia");
currency!(MOP, "Macao");
currency!(MRU, "Mauritania");
currency!(MUR, "Mauritius");
currency!(MVR, "Maldives");
currency!(MWK, "Malawi");
currency!(MXN, "Mexico");
currency!(MXV, "Mexico");
currency!(MYR, "Malaysia");
currency!(MZN, "Mozambique");
currency!(NAD, "Namibia");
currency!(NGN, "Nigeria");
currency!(NIO, "Nicaragua");
currency!(NOK, "Norway");
currency!(NPR, "Nepal");
currency!(NZD, "New Zealand");
currency!(OMR, "Oman");
currency!(PAB, "Panama");
currency!(PEN, "Peru");
currency!(PGK, "Papua New Guinea");
currency!(PHP, "Philippines (The)");
currency!(PKR, "Pakistan");
currency!(PLN, "Poland");
currency!(PYG, "Paraguay");
currency!(QAR, "Qatar");
currency!(RON, "Romania");
currency!(RSD, "Serbia");
currency!(RUB, "Russian Federation (The)");
currency!(RWF, "Rwanda");
currency!(SAR, "Saudi Arabia");
currency!(SBD, "Solomon Islands");
currency!(SCR, "Seychelles");
currency!(SDG, "Sudan (The)");
currency!(SEK, "Sweden");
currency!(SGD, "Singapore");
currency!(SHP, "Saint Helena, Ascension And Tristan Da Cunha");
currency!(SLL, "Sierra Leone");
currency!(SOS, "Somalia");
currency!(SRD, "Suriname");
currency!(SSP, "South Sudan");
currency!(STN, "Sao Tome And Principe");
currency!(SVC, "El Salvador");
currency!(SYP, "Syrian Arab Republic");
currency!(SZL, "Eswatini");
currency!(THB, "Thailand");
currency!(TJS, "Tajikistan");
currency!(TMT, "Turkmenistan");
currency!(TND, "Tunisia");
currency!(TOP, "Tonga");
currency!(TRY, "Turkey");
currency!(TTD, "Trinidad And Tobago");
currency!(TWD, "Taiwan (Province Of China)");
currency!(TZS, "Tanzania, United Republic Of");
currency!(UAH, "Ukraine");
currency!(UGX, "Uganda");
currency!(USD, "United States Of America (The)");
currency!(UYI, "Uruguay");
currency!(UYU, "Uruguay");
currency!(UYW, "Uruguay");
currency!(UZS, "Uzbekistan");
currency!(VES, "Venezuela (Bolivarian Republic Of)");
currency!(VND, "Viet Nam");
currency!(VUV, "Vanuatu");
currency!(WST, "Samoa");
currency!(XAF, "Central African Republic (The)");
currency!(XAG, "Zz11_Silver");
currency!(XAU, "Zz08_Gold");
currency!(XBA, "Zz01_Bond Markets Unit European_Eurco");
currency!(XBB, "Zz02_Bond Markets Unit European_Emu-6");
currency!(XBC, "Zz03_Bond Markets Unit European_Eua-9");
currency!(XBD, "Zz04_Bond Markets Unit European_Eua-17");
currency!(XCD, "Anguilla");
currency!(XDR, "International Monetary Fund (Imf)");
currency!(XOF, "Senegal");
currency!(XPD, "Zz09_Palladium");
currency!(XPF, "French Polynesia");
currency!(XPT, "Zz10_Platinum");
currency!(XSU, "Sistema Unitario De Compensacion Regional De Pagos");
currency!(XTS, "Zz06_Testing_Code");
currency!(
    XUA,
    "Member Countries Of The African Development Bank Group"
);
currency!(XXX, "Zz07_No_Currency");
currency!(YER, "Yemen");
currency!(ZAR, "South Africa");
currency!(ZMW, "Zambia");
currency!(ZWL, "Zimbabwe");
