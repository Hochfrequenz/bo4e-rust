#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Waehrungscode {
    #[serde(rename = "AED")]
    Aed,
    #[serde(rename = "AFN")]
    Afn,
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "AMD")]
    Amd,
    #[serde(rename = "ANG")]
    Ang,
    #[serde(rename = "AOA")]
    Aoa,
    #[serde(rename = "ARS")]
    Ars,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "AWG")]
    Awg,
    #[serde(rename = "AZN")]
    Azn,
    #[serde(rename = "BAM")]
    Bam,
    #[serde(rename = "BBD")]
    Bbd,
    #[serde(rename = "BDT")]
    Bdt,
    #[serde(rename = "BGN")]
    Bgn,
    #[serde(rename = "BHD")]
    Bhd,
    #[serde(rename = "BIF")]
    Bif,
    #[serde(rename = "BMD")]
    Bmd,
    #[serde(rename = "BND")]
    Bnd,
    #[serde(rename = "BOB")]
    Bob,
    #[serde(rename = "BOV")]
    Bov,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "BSD")]
    Bsd,
    #[serde(rename = "BTN")]
    Btn,
    #[serde(rename = "BWP")]
    Bwp,
    #[serde(rename = "BYN")]
    Byn,
    #[serde(rename = "BYR")]
    Byr,
    #[serde(rename = "BZD")]
    Bzd,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "CDF")]
    Cdf,
    #[serde(rename = "CHE")]
    Che,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "CHW")]
    Chw,
    #[serde(rename = "CLF")]
    Clf,
    #[serde(rename = "CLP")]
    Clp,
    #[serde(rename = "CNY")]
    Cny,
    #[serde(rename = "COP")]
    Cop,
    #[serde(rename = "COU")]
    Cou,
    #[serde(rename = "CRC")]
    Crc,
    #[serde(rename = "CUC")]
    Cuc,
    #[serde(rename = "CUP")]
    Cup,
    #[serde(rename = "CVE")]
    Cve,
    #[serde(rename = "CZK")]
    Czk,
    #[serde(rename = "DJF")]
    Djf,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "DOP")]
    Dop,
    #[serde(rename = "DZD")]
    Dzd,
    #[serde(rename = "EGP")]
    Egp,
    #[serde(rename = "ERN")]
    Ern,
    #[serde(rename = "ETB")]
    Etb,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "FJD")]
    Fjd,
    #[serde(rename = "FKP")]
    Fkp,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "GEL")]
    Gel,
    #[serde(rename = "GHS")]
    Ghs,
    #[serde(rename = "GIP")]
    Gip,
    #[serde(rename = "GMD")]
    Gmd,
    #[serde(rename = "GNF")]
    Gnf,
    #[serde(rename = "GTQ")]
    Gtq,
    #[serde(rename = "GYD")]
    Gyd,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "HNL")]
    Hnl,
    #[serde(rename = "HRK")]
    Hrk,
    #[serde(rename = "HTG")]
    Htg,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "IDR")]
    Idr,
    #[serde(rename = "ILS")]
    Ils,
    #[serde(rename = "INR")]
    Inr,
    #[serde(rename = "IQD")]
    Iqd,
    #[serde(rename = "IRR")]
    Irr,
    #[serde(rename = "ISK")]
    Isk,
    #[serde(rename = "JMD")]
    Jmd,
    #[serde(rename = "JOD")]
    Jod,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "KES")]
    Kes,
    #[serde(rename = "KGS")]
    Kgs,
    #[serde(rename = "KHR")]
    Khr,
    #[serde(rename = "KMF")]
    Kmf,
    #[serde(rename = "KPW")]
    Kpw,
    #[serde(rename = "KRW")]
    Krw,
    #[serde(rename = "KWD")]
    Kwd,
    #[serde(rename = "KYD")]
    Kyd,
    #[serde(rename = "KZT")]
    Kzt,
    #[serde(rename = "LAK")]
    Lak,
    #[serde(rename = "LBP")]
    Lbp,
    #[serde(rename = "LKR")]
    Lkr,
    #[serde(rename = "LRD")]
    Lrd,
    #[serde(rename = "LSL")]
    Lsl,
    #[serde(rename = "LTL")]
    Ltl,
    #[serde(rename = "LYD")]
    Lyd,
    #[serde(rename = "MAD")]
    Mad,
    #[serde(rename = "MDL")]
    Mdl,
    #[serde(rename = "MGA")]
    Mga,
    #[serde(rename = "MKD")]
    Mkd,
    #[serde(rename = "MMK")]
    Mmk,
    #[serde(rename = "MNT")]
    Mnt,
    #[serde(rename = "MOP")]
    Mop,
    #[serde(rename = "MRO")]
    Mro,
    #[serde(rename = "MUR")]
    Mur,
    #[serde(rename = "MVR")]
    Mvr,
    #[serde(rename = "MWK")]
    Mwk,
    #[serde(rename = "MXN")]
    Mxn,
    #[serde(rename = "MXV")]
    UDI,
    #[serde(rename = "MYR")]
    Myr,
    #[serde(rename = "MZN")]
    Mzn,
    #[serde(rename = "NAD")]
    Nad,
    #[serde(rename = "NGN")]
    Ngn,
    #[serde(rename = "NIO")]
    Nio,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "NPR")]
    Npr,
    #[serde(rename = "NZD")]
    Nzd,
    #[serde(rename = "OMR")]
    Omr,
    #[serde(rename = "PAB")]
    Pab,
    #[serde(rename = "PEN")]
    Pen,
    #[serde(rename = "PGK")]
    Pgk,
    #[serde(rename = "PHP")]
    Php,
    #[serde(rename = "PKR")]
    Pkr,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "PYG")]
    Pyg,
    #[serde(rename = "QAR")]
    Qar,
    #[serde(rename = "RON")]
    Ron,
    #[serde(rename = "RSD")]
    Rsd,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "RUR")]
    Rur,
    #[serde(rename = "RWF")]
    Rwf,
    #[serde(rename = "SAR")]
    Sar,
    #[serde(rename = "SBD")]
    Sbd,
    #[serde(rename = "SCR")]
    Scr,
    #[serde(rename = "SDG")]
    Sdg,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "SGD")]
    Sgd,
    #[serde(rename = "SHP")]
    Shp,
    #[serde(rename = "SLL")]
    Sll,
    #[serde(rename = "SOS")]
    Sos,
    #[serde(rename = "SRD")]
    Srd,
    #[serde(rename = "SSP")]
    Ssp,
    #[serde(rename = "STD")]
    Std,
    #[serde(rename = "SVC")]
    Svc,
    #[serde(rename = "SYP")]
    Syp,
    #[serde(rename = "SZL")]
    Szl,
    #[serde(rename = "THB")]
    Thb,
    #[serde(rename = "TJS")]
    Tjs,
    #[serde(rename = "TMT")]
    Tmt,
    #[serde(rename = "TND")]
    Tnd,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "TRY")]
    Try,
    #[serde(rename = "TTD")]
    Ttd,
    #[serde(rename = "TWD")]
    Twd,
    #[serde(rename = "TZS")]
    Tzs,
    #[serde(rename = "UAH")]
    Uah,
    #[serde(rename = "UGX")]
    Ugx,
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "USN")]
    NextDay,
    #[serde(rename = "USS")]
    SameDay,
    #[serde(rename = "UYI")]
    Uyi,
    #[serde(rename = "UYU")]
    Uyu,
    #[serde(rename = "UZS")]
    Uzs,
    #[serde(rename = "VEF")]
    Vef,
    #[serde(rename = "VND")]
    Vnd,
    #[serde(rename = "VUV")]
    Vuv,
    #[serde(rename = "WST")]
    Wst,
    #[serde(rename = "XAF")]
    Xaf,
    #[serde(rename = "XAG")]
    Xag,
    #[serde(rename = "XAU")]
    Xau,
    #[serde(rename = "XBA")]
    EURCO,
    #[serde(rename = "XBB")]
    EMU6,
    #[serde(rename = "XBC")]
    EUA9,
    #[serde(rename = "XBD")]
    EUA17,
    #[serde(rename = "XCD")]
    Xcd,
    #[serde(rename = "XDR")]
    SpecialDrawingRight,
    #[serde(rename = "XOF")]
    Xof,
    #[serde(rename = "XPD")]
    Xpd,
    #[serde(rename = "XPF")]
    Xpf,
    #[serde(rename = "XPT")]
    Xpt,
    #[serde(rename = "XSU")]
    Xsu,
    #[serde(rename = "XTS")]
    Xts,
    #[serde(rename = "XUA")]
    Xua,
    #[serde(rename = "XXX")]
    Xxx,
    #[serde(rename = "YER")]
    Yer,
    #[serde(rename = "ZAR")]
    Zar,
    #[serde(rename = "ZMW")]
    Zmw,
    #[serde(rename = "ZWL")]
    Zwl,
}
impl From<bo4e_core::enums::Currency> for Waehrungscode {
    fn from(v: bo4e_core::enums::Currency) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            bo4e_core::enums::Currency::Aed => Waehrungscode::Aed,
            bo4e_core::enums::Currency::Afn => Waehrungscode::Afn,
            bo4e_core::enums::Currency::All => Waehrungscode::All,
            bo4e_core::enums::Currency::Amd => Waehrungscode::Amd,
            bo4e_core::enums::Currency::Ang => Waehrungscode::Ang,
            bo4e_core::enums::Currency::Aoa => Waehrungscode::Aoa,
            bo4e_core::enums::Currency::Ars => Waehrungscode::Ars,
            bo4e_core::enums::Currency::Aud => Waehrungscode::Aud,
            bo4e_core::enums::Currency::Awg => Waehrungscode::Awg,
            bo4e_core::enums::Currency::Azn => Waehrungscode::Azn,
            bo4e_core::enums::Currency::Bam => Waehrungscode::Bam,
            bo4e_core::enums::Currency::Bbd => Waehrungscode::Bbd,
            bo4e_core::enums::Currency::Bdt => Waehrungscode::Bdt,
            bo4e_core::enums::Currency::Bgn => Waehrungscode::Bgn,
            bo4e_core::enums::Currency::Bhd => Waehrungscode::Bhd,
            bo4e_core::enums::Currency::Bif => Waehrungscode::Bif,
            bo4e_core::enums::Currency::Bmd => Waehrungscode::Bmd,
            bo4e_core::enums::Currency::Bnd => Waehrungscode::Bnd,
            bo4e_core::enums::Currency::Bob => Waehrungscode::Bob,
            bo4e_core::enums::Currency::Bov => Waehrungscode::Bov,
            bo4e_core::enums::Currency::Brl => Waehrungscode::Brl,
            bo4e_core::enums::Currency::Bsd => Waehrungscode::Bsd,
            bo4e_core::enums::Currency::Btn => Waehrungscode::Btn,
            bo4e_core::enums::Currency::Bwp => Waehrungscode::Bwp,
            bo4e_core::enums::Currency::Byn => Waehrungscode::Byn,
            bo4e_core::enums::Currency::Byr => Waehrungscode::Byr,
            bo4e_core::enums::Currency::Bzd => Waehrungscode::Bzd,
            bo4e_core::enums::Currency::Cad => Waehrungscode::Cad,
            bo4e_core::enums::Currency::Cdf => Waehrungscode::Cdf,
            bo4e_core::enums::Currency::Che => Waehrungscode::Che,
            bo4e_core::enums::Currency::Chf => Waehrungscode::Chf,
            bo4e_core::enums::Currency::Chw => Waehrungscode::Chw,
            bo4e_core::enums::Currency::Clf => Waehrungscode::Clf,
            bo4e_core::enums::Currency::Clp => Waehrungscode::Clp,
            bo4e_core::enums::Currency::Cny => Waehrungscode::Cny,
            bo4e_core::enums::Currency::Cop => Waehrungscode::Cop,
            bo4e_core::enums::Currency::Cou => Waehrungscode::Cou,
            bo4e_core::enums::Currency::Crc => Waehrungscode::Crc,
            bo4e_core::enums::Currency::Cuc => Waehrungscode::Cuc,
            bo4e_core::enums::Currency::Cup => Waehrungscode::Cup,
            bo4e_core::enums::Currency::Cve => Waehrungscode::Cve,
            bo4e_core::enums::Currency::Czk => Waehrungscode::Czk,
            bo4e_core::enums::Currency::Djf => Waehrungscode::Djf,
            bo4e_core::enums::Currency::Dkk => Waehrungscode::Dkk,
            bo4e_core::enums::Currency::Dop => Waehrungscode::Dop,
            bo4e_core::enums::Currency::Dzd => Waehrungscode::Dzd,
            bo4e_core::enums::Currency::Egp => Waehrungscode::Egp,
            bo4e_core::enums::Currency::Ern => Waehrungscode::Ern,
            bo4e_core::enums::Currency::Etb => Waehrungscode::Etb,
            bo4e_core::enums::Currency::Eur => Waehrungscode::Eur,
            bo4e_core::enums::Currency::Fjd => Waehrungscode::Fjd,
            bo4e_core::enums::Currency::Fkp => Waehrungscode::Fkp,
            bo4e_core::enums::Currency::Gbp => Waehrungscode::Gbp,
            bo4e_core::enums::Currency::Gel => Waehrungscode::Gel,
            bo4e_core::enums::Currency::Ghs => Waehrungscode::Ghs,
            bo4e_core::enums::Currency::Gip => Waehrungscode::Gip,
            bo4e_core::enums::Currency::Gmd => Waehrungscode::Gmd,
            bo4e_core::enums::Currency::Gnf => Waehrungscode::Gnf,
            bo4e_core::enums::Currency::Gtq => Waehrungscode::Gtq,
            bo4e_core::enums::Currency::Gyd => Waehrungscode::Gyd,
            bo4e_core::enums::Currency::Hkd => Waehrungscode::Hkd,
            bo4e_core::enums::Currency::Hnl => Waehrungscode::Hnl,
            bo4e_core::enums::Currency::Hrk => Waehrungscode::Hrk,
            bo4e_core::enums::Currency::Htg => Waehrungscode::Htg,
            bo4e_core::enums::Currency::Huf => Waehrungscode::Huf,
            bo4e_core::enums::Currency::Idr => Waehrungscode::Idr,
            bo4e_core::enums::Currency::Ils => Waehrungscode::Ils,
            bo4e_core::enums::Currency::Inr => Waehrungscode::Inr,
            bo4e_core::enums::Currency::Iqd => Waehrungscode::Iqd,
            bo4e_core::enums::Currency::Irr => Waehrungscode::Irr,
            bo4e_core::enums::Currency::Isk => Waehrungscode::Isk,
            bo4e_core::enums::Currency::Jmd => Waehrungscode::Jmd,
            bo4e_core::enums::Currency::Jod => Waehrungscode::Jod,
            bo4e_core::enums::Currency::Jpy => Waehrungscode::Jpy,
            bo4e_core::enums::Currency::Kes => Waehrungscode::Kes,
            bo4e_core::enums::Currency::Kgs => Waehrungscode::Kgs,
            bo4e_core::enums::Currency::Khr => Waehrungscode::Khr,
            bo4e_core::enums::Currency::Kmf => Waehrungscode::Kmf,
            bo4e_core::enums::Currency::Kpw => Waehrungscode::Kpw,
            bo4e_core::enums::Currency::Krw => Waehrungscode::Krw,
            bo4e_core::enums::Currency::Kwd => Waehrungscode::Kwd,
            bo4e_core::enums::Currency::Kyd => Waehrungscode::Kyd,
            bo4e_core::enums::Currency::Kzt => Waehrungscode::Kzt,
            bo4e_core::enums::Currency::Lak => Waehrungscode::Lak,
            bo4e_core::enums::Currency::Lbp => Waehrungscode::Lbp,
            bo4e_core::enums::Currency::Lkr => Waehrungscode::Lkr,
            bo4e_core::enums::Currency::Lrd => Waehrungscode::Lrd,
            bo4e_core::enums::Currency::Lsl => Waehrungscode::Lsl,
            bo4e_core::enums::Currency::Ltl => Waehrungscode::Ltl,
            bo4e_core::enums::Currency::Lyd => Waehrungscode::Lyd,
            bo4e_core::enums::Currency::Mad => Waehrungscode::Mad,
            bo4e_core::enums::Currency::Mdl => Waehrungscode::Mdl,
            bo4e_core::enums::Currency::Mga => Waehrungscode::Mga,
            bo4e_core::enums::Currency::Mkd => Waehrungscode::Mkd,
            bo4e_core::enums::Currency::Mmk => Waehrungscode::Mmk,
            bo4e_core::enums::Currency::Mnt => Waehrungscode::Mnt,
            bo4e_core::enums::Currency::Mop => Waehrungscode::Mop,
            bo4e_core::enums::Currency::Mro => Waehrungscode::Mro,
            bo4e_core::enums::Currency::Mur => Waehrungscode::Mur,
            bo4e_core::enums::Currency::Mvr => Waehrungscode::Mvr,
            bo4e_core::enums::Currency::Mwk => Waehrungscode::Mwk,
            bo4e_core::enums::Currency::Mxn => Waehrungscode::Mxn,
            bo4e_core::enums::Currency::Mxv => Waehrungscode::UDI,
            bo4e_core::enums::Currency::Myr => Waehrungscode::Myr,
            bo4e_core::enums::Currency::Mzn => Waehrungscode::Mzn,
            bo4e_core::enums::Currency::Nad => Waehrungscode::Nad,
            bo4e_core::enums::Currency::Ngn => Waehrungscode::Ngn,
            bo4e_core::enums::Currency::Nio => Waehrungscode::Nio,
            bo4e_core::enums::Currency::Nok => Waehrungscode::Nok,
            bo4e_core::enums::Currency::Npr => Waehrungscode::Npr,
            bo4e_core::enums::Currency::Nzd => Waehrungscode::Nzd,
            bo4e_core::enums::Currency::Omr => Waehrungscode::Omr,
            bo4e_core::enums::Currency::Pab => Waehrungscode::Pab,
            bo4e_core::enums::Currency::Pen => Waehrungscode::Pen,
            bo4e_core::enums::Currency::Pgk => Waehrungscode::Pgk,
            bo4e_core::enums::Currency::Php => Waehrungscode::Php,
            bo4e_core::enums::Currency::Pkr => Waehrungscode::Pkr,
            bo4e_core::enums::Currency::Pln => Waehrungscode::Pln,
            bo4e_core::enums::Currency::Pyg => Waehrungscode::Pyg,
            bo4e_core::enums::Currency::Qar => Waehrungscode::Qar,
            bo4e_core::enums::Currency::Ron => Waehrungscode::Ron,
            bo4e_core::enums::Currency::Rsd => Waehrungscode::Rsd,
            bo4e_core::enums::Currency::Rub => Waehrungscode::Rub,
            bo4e_core::enums::Currency::Rur => Waehrungscode::Rur,
            bo4e_core::enums::Currency::Rwf => Waehrungscode::Rwf,
            bo4e_core::enums::Currency::Sar => Waehrungscode::Sar,
            bo4e_core::enums::Currency::Sbd => Waehrungscode::Sbd,
            bo4e_core::enums::Currency::Scr => Waehrungscode::Scr,
            bo4e_core::enums::Currency::Sdg => Waehrungscode::Sdg,
            bo4e_core::enums::Currency::Sek => Waehrungscode::Sek,
            bo4e_core::enums::Currency::Sgd => Waehrungscode::Sgd,
            bo4e_core::enums::Currency::Shp => Waehrungscode::Shp,
            bo4e_core::enums::Currency::Sll => Waehrungscode::Sll,
            bo4e_core::enums::Currency::Sos => Waehrungscode::Sos,
            bo4e_core::enums::Currency::Srd => Waehrungscode::Srd,
            bo4e_core::enums::Currency::Ssp => Waehrungscode::Ssp,
            bo4e_core::enums::Currency::Std => Waehrungscode::Std,
            bo4e_core::enums::Currency::Svc => Waehrungscode::Svc,
            bo4e_core::enums::Currency::Syp => Waehrungscode::Syp,
            bo4e_core::enums::Currency::Szl => Waehrungscode::Szl,
            bo4e_core::enums::Currency::Thb => Waehrungscode::Thb,
            bo4e_core::enums::Currency::Tjs => Waehrungscode::Tjs,
            bo4e_core::enums::Currency::Tmt => Waehrungscode::Tmt,
            bo4e_core::enums::Currency::Tnd => Waehrungscode::Tnd,
            bo4e_core::enums::Currency::Top => Waehrungscode::Top,
            bo4e_core::enums::Currency::Try => Waehrungscode::Try,
            bo4e_core::enums::Currency::Ttd => Waehrungscode::Ttd,
            bo4e_core::enums::Currency::Twd => Waehrungscode::Twd,
            bo4e_core::enums::Currency::Tzs => Waehrungscode::Tzs,
            bo4e_core::enums::Currency::Uah => Waehrungscode::Uah,
            bo4e_core::enums::Currency::Ugx => Waehrungscode::Ugx,
            bo4e_core::enums::Currency::Usd => Waehrungscode::Usd,
            bo4e_core::enums::Currency::Usn => Waehrungscode::NextDay,
            bo4e_core::enums::Currency::Uss => Waehrungscode::SameDay,
            bo4e_core::enums::Currency::Uyi => Waehrungscode::Uyi,
            bo4e_core::enums::Currency::Uyu => Waehrungscode::Uyu,
            bo4e_core::enums::Currency::Uzs => Waehrungscode::Uzs,
            bo4e_core::enums::Currency::Vef => Waehrungscode::Vef,
            bo4e_core::enums::Currency::Vnd => Waehrungscode::Vnd,
            bo4e_core::enums::Currency::Vuv => Waehrungscode::Vuv,
            bo4e_core::enums::Currency::Wst => Waehrungscode::Wst,
            bo4e_core::enums::Currency::Xaf => Waehrungscode::Xaf,
            bo4e_core::enums::Currency::Xag => Waehrungscode::Xag,
            bo4e_core::enums::Currency::Xau => Waehrungscode::Xau,
            bo4e_core::enums::Currency::Xba => Waehrungscode::EURCO,
            bo4e_core::enums::Currency::Xbb => Waehrungscode::EMU6,
            bo4e_core::enums::Currency::Xbc => Waehrungscode::EUA9,
            bo4e_core::enums::Currency::Xbd => Waehrungscode::EUA17,
            bo4e_core::enums::Currency::Xcd => Waehrungscode::Xcd,
            bo4e_core::enums::Currency::Xdr => Waehrungscode::SpecialDrawingRight,
            bo4e_core::enums::Currency::Xof => Waehrungscode::Xof,
            bo4e_core::enums::Currency::Xpd => Waehrungscode::Xpd,
            bo4e_core::enums::Currency::Xpf => Waehrungscode::Xpf,
            bo4e_core::enums::Currency::Xpt => Waehrungscode::Xpt,
            bo4e_core::enums::Currency::Xsu => Waehrungscode::Xsu,
            bo4e_core::enums::Currency::Xts => Waehrungscode::Xts,
            bo4e_core::enums::Currency::Xua => Waehrungscode::Xua,
            bo4e_core::enums::Currency::Xxx => Waehrungscode::Xxx,
            bo4e_core::enums::Currency::Yer => Waehrungscode::Yer,
            bo4e_core::enums::Currency::Zar => Waehrungscode::Zar,
            bo4e_core::enums::Currency::Zmw => Waehrungscode::Zmw,
            bo4e_core::enums::Currency::Zwl => Waehrungscode::Zwl,
            _ => panic!("Unknown {} variant", stringify!(Currency)),
        }
    }
}
impl From<Waehrungscode> for bo4e_core::enums::Currency {
    fn from(v: Waehrungscode) -> Self {
        #[allow(unreachable_patterns)]
        match v {
            Waehrungscode::Aed => bo4e_core::enums::Currency::Aed,
            Waehrungscode::Afn => bo4e_core::enums::Currency::Afn,
            Waehrungscode::All => bo4e_core::enums::Currency::All,
            Waehrungscode::Amd => bo4e_core::enums::Currency::Amd,
            Waehrungscode::Ang => bo4e_core::enums::Currency::Ang,
            Waehrungscode::Aoa => bo4e_core::enums::Currency::Aoa,
            Waehrungscode::Ars => bo4e_core::enums::Currency::Ars,
            Waehrungscode::Aud => bo4e_core::enums::Currency::Aud,
            Waehrungscode::Awg => bo4e_core::enums::Currency::Awg,
            Waehrungscode::Azn => bo4e_core::enums::Currency::Azn,
            Waehrungscode::Bam => bo4e_core::enums::Currency::Bam,
            Waehrungscode::Bbd => bo4e_core::enums::Currency::Bbd,
            Waehrungscode::Bdt => bo4e_core::enums::Currency::Bdt,
            Waehrungscode::Bgn => bo4e_core::enums::Currency::Bgn,
            Waehrungscode::Bhd => bo4e_core::enums::Currency::Bhd,
            Waehrungscode::Bif => bo4e_core::enums::Currency::Bif,
            Waehrungscode::Bmd => bo4e_core::enums::Currency::Bmd,
            Waehrungscode::Bnd => bo4e_core::enums::Currency::Bnd,
            Waehrungscode::Bob => bo4e_core::enums::Currency::Bob,
            Waehrungscode::Bov => bo4e_core::enums::Currency::Bov,
            Waehrungscode::Brl => bo4e_core::enums::Currency::Brl,
            Waehrungscode::Bsd => bo4e_core::enums::Currency::Bsd,
            Waehrungscode::Btn => bo4e_core::enums::Currency::Btn,
            Waehrungscode::Bwp => bo4e_core::enums::Currency::Bwp,
            Waehrungscode::Byn => bo4e_core::enums::Currency::Byn,
            Waehrungscode::Byr => bo4e_core::enums::Currency::Byr,
            Waehrungscode::Bzd => bo4e_core::enums::Currency::Bzd,
            Waehrungscode::Cad => bo4e_core::enums::Currency::Cad,
            Waehrungscode::Cdf => bo4e_core::enums::Currency::Cdf,
            Waehrungscode::Che => bo4e_core::enums::Currency::Che,
            Waehrungscode::Chf => bo4e_core::enums::Currency::Chf,
            Waehrungscode::Chw => bo4e_core::enums::Currency::Chw,
            Waehrungscode::Clf => bo4e_core::enums::Currency::Clf,
            Waehrungscode::Clp => bo4e_core::enums::Currency::Clp,
            Waehrungscode::Cny => bo4e_core::enums::Currency::Cny,
            Waehrungscode::Cop => bo4e_core::enums::Currency::Cop,
            Waehrungscode::Cou => bo4e_core::enums::Currency::Cou,
            Waehrungscode::Crc => bo4e_core::enums::Currency::Crc,
            Waehrungscode::Cuc => bo4e_core::enums::Currency::Cuc,
            Waehrungscode::Cup => bo4e_core::enums::Currency::Cup,
            Waehrungscode::Cve => bo4e_core::enums::Currency::Cve,
            Waehrungscode::Czk => bo4e_core::enums::Currency::Czk,
            Waehrungscode::Djf => bo4e_core::enums::Currency::Djf,
            Waehrungscode::Dkk => bo4e_core::enums::Currency::Dkk,
            Waehrungscode::Dop => bo4e_core::enums::Currency::Dop,
            Waehrungscode::Dzd => bo4e_core::enums::Currency::Dzd,
            Waehrungscode::Egp => bo4e_core::enums::Currency::Egp,
            Waehrungscode::Ern => bo4e_core::enums::Currency::Ern,
            Waehrungscode::Etb => bo4e_core::enums::Currency::Etb,
            Waehrungscode::Eur => bo4e_core::enums::Currency::Eur,
            Waehrungscode::Fjd => bo4e_core::enums::Currency::Fjd,
            Waehrungscode::Fkp => bo4e_core::enums::Currency::Fkp,
            Waehrungscode::Gbp => bo4e_core::enums::Currency::Gbp,
            Waehrungscode::Gel => bo4e_core::enums::Currency::Gel,
            Waehrungscode::Ghs => bo4e_core::enums::Currency::Ghs,
            Waehrungscode::Gip => bo4e_core::enums::Currency::Gip,
            Waehrungscode::Gmd => bo4e_core::enums::Currency::Gmd,
            Waehrungscode::Gnf => bo4e_core::enums::Currency::Gnf,
            Waehrungscode::Gtq => bo4e_core::enums::Currency::Gtq,
            Waehrungscode::Gyd => bo4e_core::enums::Currency::Gyd,
            Waehrungscode::Hkd => bo4e_core::enums::Currency::Hkd,
            Waehrungscode::Hnl => bo4e_core::enums::Currency::Hnl,
            Waehrungscode::Hrk => bo4e_core::enums::Currency::Hrk,
            Waehrungscode::Htg => bo4e_core::enums::Currency::Htg,
            Waehrungscode::Huf => bo4e_core::enums::Currency::Huf,
            Waehrungscode::Idr => bo4e_core::enums::Currency::Idr,
            Waehrungscode::Ils => bo4e_core::enums::Currency::Ils,
            Waehrungscode::Inr => bo4e_core::enums::Currency::Inr,
            Waehrungscode::Iqd => bo4e_core::enums::Currency::Iqd,
            Waehrungscode::Irr => bo4e_core::enums::Currency::Irr,
            Waehrungscode::Isk => bo4e_core::enums::Currency::Isk,
            Waehrungscode::Jmd => bo4e_core::enums::Currency::Jmd,
            Waehrungscode::Jod => bo4e_core::enums::Currency::Jod,
            Waehrungscode::Jpy => bo4e_core::enums::Currency::Jpy,
            Waehrungscode::Kes => bo4e_core::enums::Currency::Kes,
            Waehrungscode::Kgs => bo4e_core::enums::Currency::Kgs,
            Waehrungscode::Khr => bo4e_core::enums::Currency::Khr,
            Waehrungscode::Kmf => bo4e_core::enums::Currency::Kmf,
            Waehrungscode::Kpw => bo4e_core::enums::Currency::Kpw,
            Waehrungscode::Krw => bo4e_core::enums::Currency::Krw,
            Waehrungscode::Kwd => bo4e_core::enums::Currency::Kwd,
            Waehrungscode::Kyd => bo4e_core::enums::Currency::Kyd,
            Waehrungscode::Kzt => bo4e_core::enums::Currency::Kzt,
            Waehrungscode::Lak => bo4e_core::enums::Currency::Lak,
            Waehrungscode::Lbp => bo4e_core::enums::Currency::Lbp,
            Waehrungscode::Lkr => bo4e_core::enums::Currency::Lkr,
            Waehrungscode::Lrd => bo4e_core::enums::Currency::Lrd,
            Waehrungscode::Lsl => bo4e_core::enums::Currency::Lsl,
            Waehrungscode::Ltl => bo4e_core::enums::Currency::Ltl,
            Waehrungscode::Lyd => bo4e_core::enums::Currency::Lyd,
            Waehrungscode::Mad => bo4e_core::enums::Currency::Mad,
            Waehrungscode::Mdl => bo4e_core::enums::Currency::Mdl,
            Waehrungscode::Mga => bo4e_core::enums::Currency::Mga,
            Waehrungscode::Mkd => bo4e_core::enums::Currency::Mkd,
            Waehrungscode::Mmk => bo4e_core::enums::Currency::Mmk,
            Waehrungscode::Mnt => bo4e_core::enums::Currency::Mnt,
            Waehrungscode::Mop => bo4e_core::enums::Currency::Mop,
            Waehrungscode::Mro => bo4e_core::enums::Currency::Mro,
            Waehrungscode::Mur => bo4e_core::enums::Currency::Mur,
            Waehrungscode::Mvr => bo4e_core::enums::Currency::Mvr,
            Waehrungscode::Mwk => bo4e_core::enums::Currency::Mwk,
            Waehrungscode::Mxn => bo4e_core::enums::Currency::Mxn,
            Waehrungscode::UDI => bo4e_core::enums::Currency::Mxv,
            Waehrungscode::Myr => bo4e_core::enums::Currency::Myr,
            Waehrungscode::Mzn => bo4e_core::enums::Currency::Mzn,
            Waehrungscode::Nad => bo4e_core::enums::Currency::Nad,
            Waehrungscode::Ngn => bo4e_core::enums::Currency::Ngn,
            Waehrungscode::Nio => bo4e_core::enums::Currency::Nio,
            Waehrungscode::Nok => bo4e_core::enums::Currency::Nok,
            Waehrungscode::Npr => bo4e_core::enums::Currency::Npr,
            Waehrungscode::Nzd => bo4e_core::enums::Currency::Nzd,
            Waehrungscode::Omr => bo4e_core::enums::Currency::Omr,
            Waehrungscode::Pab => bo4e_core::enums::Currency::Pab,
            Waehrungscode::Pen => bo4e_core::enums::Currency::Pen,
            Waehrungscode::Pgk => bo4e_core::enums::Currency::Pgk,
            Waehrungscode::Php => bo4e_core::enums::Currency::Php,
            Waehrungscode::Pkr => bo4e_core::enums::Currency::Pkr,
            Waehrungscode::Pln => bo4e_core::enums::Currency::Pln,
            Waehrungscode::Pyg => bo4e_core::enums::Currency::Pyg,
            Waehrungscode::Qar => bo4e_core::enums::Currency::Qar,
            Waehrungscode::Ron => bo4e_core::enums::Currency::Ron,
            Waehrungscode::Rsd => bo4e_core::enums::Currency::Rsd,
            Waehrungscode::Rub => bo4e_core::enums::Currency::Rub,
            Waehrungscode::Rur => bo4e_core::enums::Currency::Rur,
            Waehrungscode::Rwf => bo4e_core::enums::Currency::Rwf,
            Waehrungscode::Sar => bo4e_core::enums::Currency::Sar,
            Waehrungscode::Sbd => bo4e_core::enums::Currency::Sbd,
            Waehrungscode::Scr => bo4e_core::enums::Currency::Scr,
            Waehrungscode::Sdg => bo4e_core::enums::Currency::Sdg,
            Waehrungscode::Sek => bo4e_core::enums::Currency::Sek,
            Waehrungscode::Sgd => bo4e_core::enums::Currency::Sgd,
            Waehrungscode::Shp => bo4e_core::enums::Currency::Shp,
            Waehrungscode::Sll => bo4e_core::enums::Currency::Sll,
            Waehrungscode::Sos => bo4e_core::enums::Currency::Sos,
            Waehrungscode::Srd => bo4e_core::enums::Currency::Srd,
            Waehrungscode::Ssp => bo4e_core::enums::Currency::Ssp,
            Waehrungscode::Std => bo4e_core::enums::Currency::Std,
            Waehrungscode::Svc => bo4e_core::enums::Currency::Svc,
            Waehrungscode::Syp => bo4e_core::enums::Currency::Syp,
            Waehrungscode::Szl => bo4e_core::enums::Currency::Szl,
            Waehrungscode::Thb => bo4e_core::enums::Currency::Thb,
            Waehrungscode::Tjs => bo4e_core::enums::Currency::Tjs,
            Waehrungscode::Tmt => bo4e_core::enums::Currency::Tmt,
            Waehrungscode::Tnd => bo4e_core::enums::Currency::Tnd,
            Waehrungscode::Top => bo4e_core::enums::Currency::Top,
            Waehrungscode::Try => bo4e_core::enums::Currency::Try,
            Waehrungscode::Ttd => bo4e_core::enums::Currency::Ttd,
            Waehrungscode::Twd => bo4e_core::enums::Currency::Twd,
            Waehrungscode::Tzs => bo4e_core::enums::Currency::Tzs,
            Waehrungscode::Uah => bo4e_core::enums::Currency::Uah,
            Waehrungscode::Ugx => bo4e_core::enums::Currency::Ugx,
            Waehrungscode::Usd => bo4e_core::enums::Currency::Usd,
            Waehrungscode::NextDay => bo4e_core::enums::Currency::Usn,
            Waehrungscode::SameDay => bo4e_core::enums::Currency::Uss,
            Waehrungscode::Uyi => bo4e_core::enums::Currency::Uyi,
            Waehrungscode::Uyu => bo4e_core::enums::Currency::Uyu,
            Waehrungscode::Uzs => bo4e_core::enums::Currency::Uzs,
            Waehrungscode::Vef => bo4e_core::enums::Currency::Vef,
            Waehrungscode::Vnd => bo4e_core::enums::Currency::Vnd,
            Waehrungscode::Vuv => bo4e_core::enums::Currency::Vuv,
            Waehrungscode::Wst => bo4e_core::enums::Currency::Wst,
            Waehrungscode::Xaf => bo4e_core::enums::Currency::Xaf,
            Waehrungscode::Xag => bo4e_core::enums::Currency::Xag,
            Waehrungscode::Xau => bo4e_core::enums::Currency::Xau,
            Waehrungscode::EURCO => bo4e_core::enums::Currency::Xba,
            Waehrungscode::EMU6 => bo4e_core::enums::Currency::Xbb,
            Waehrungscode::EUA9 => bo4e_core::enums::Currency::Xbc,
            Waehrungscode::EUA17 => bo4e_core::enums::Currency::Xbd,
            Waehrungscode::Xcd => bo4e_core::enums::Currency::Xcd,
            Waehrungscode::SpecialDrawingRight => bo4e_core::enums::Currency::Xdr,
            Waehrungscode::Xof => bo4e_core::enums::Currency::Xof,
            Waehrungscode::Xpd => bo4e_core::enums::Currency::Xpd,
            Waehrungscode::Xpf => bo4e_core::enums::Currency::Xpf,
            Waehrungscode::Xpt => bo4e_core::enums::Currency::Xpt,
            Waehrungscode::Xsu => bo4e_core::enums::Currency::Xsu,
            Waehrungscode::Xts => bo4e_core::enums::Currency::Xts,
            Waehrungscode::Xua => bo4e_core::enums::Currency::Xua,
            Waehrungscode::Xxx => bo4e_core::enums::Currency::Xxx,
            Waehrungscode::Yer => bo4e_core::enums::Currency::Yer,
            Waehrungscode::Zar => bo4e_core::enums::Currency::Zar,
            Waehrungscode::Zmw => bo4e_core::enums::Currency::Zmw,
            Waehrungscode::Zwl => bo4e_core::enums::Currency::Zwl,
        }
    }
}
