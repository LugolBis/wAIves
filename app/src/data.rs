#[derive(Debug, Clone, Copy)]
/// Countries Abreviations
pub struct CA<'a>(pub &'a str, pub &'a str);

#[derive(Debug, Clone, Copy)]
/// Final Centroids from the clustering
pub struct Centroid<'a>(pub &'a str, pub Coordinate);

#[derive(Debug, Clone, Copy)]
pub struct Coordinate(pub f64, pub f64);

/// This constant contains the list of the countries and their abreviation
pub const COUNTRIES : [CA;141] = [
    CA("France","FR"),
    CA("Germany","DE"),
    CA("Poland","PL"),
    CA("Albania","AL"),
    CA("Russia","RU"),
    CA("Greece","GR"),
    CA("Portugal","PT"),
    CA("Belgium","BE"),
    CA("Iceland","IS"),
    CA("Romania","RO"),
    CA("Bulgaria","BG"),
    CA("Ireland","IE"),
    CA("Croatia","HR"),
    CA("Italy","IT"),
    CA("Spain","ES"),
    CA("Cyprus","CY"),
    CA("Latvia","LV"),
    CA("Sweden","SE"),
    CA("Denmark","DK"),
    CA("Lithuania","LT"),
    CA("Ukraine","UA"),
    CA("Estonia","EE"),
    CA("Malta","MT"),
    CA("United Kingdom","GB"),
    CA("Faroe Islands","FO"),
    CA("Netherlands","NL"),
    CA("Finland","FI"),
    CA("Norway","NO"),
    CA("Anguilla","AI"),
    CA("El Salvador","SV"),
    CA("Nicaragua","NI"),
    CA("Aruba","AW"),
    CA("Grenada","GD"),
    CA("Panama","PA"),
    CA("Bahamas","BS"),
    CA("Guadeloupe","GP"),
    CA("Puerto Rico","PR"),
    CA("Barbados","BB"),
    CA("Guatemala","GT"),
    CA("Saint Barthélemy","BL"),
    CA("Belize","BZ"),
    CA("Haiti","HT"),
    CA("Saint Kitts and Nevis","KN"),
    CA("Bermuda","BM"),
    CA("Honduras","HN"),
    CA("Saint Lucia","LC"),
    CA("British Virgin Islands","VG"),
    CA("Jamaica","JM"),
    CA("Trinidad and Tobago","TT"),
    CA("Canada","CA"),
    CA("Martinique","MQ"),
    CA("Turks and Caicos Islands","TC"),
    CA("Cayman Islands","KY"),
    CA("Mexico","MX"),
    CA("Costa Rica","CR"),
    CA("Montserrat","MS"),
    CA("Dominican Republic","DO"),
    CA("USA","US"),
    CA("Micronesia","FM"),
    CA("Fikland Islands","FK"),
    CA("Reunion","RE"),
    CA("Turkey","TR"),
    CA("American Samoa","AS"),
    CA("Kiribati","KI"),
    CA("Papua New Guinea","PG"),
    CA("Cook Islands","CK"),
    CA("Samoa","WS"),
    CA("New Caledonia","NC"),
    CA("Solomon Islands","SB"),
    CA("Fiji","FJ"),
    CA("New Zealand","NZ"),
    CA("Tokelau","TK"),
    CA("French Polynesia","PF"),
    CA("Northern Mariana Islands","MP"),
    CA("Tonga","TO"),
    CA("Guam","GU"),
    CA("Palau","PW"),
    CA("Vanuatu","VU"),
    CA("Australia","AU"),
    CA("Argentina","AR"),
    CA("Ecuador","EC"),
    CA("Uruguay","UY"),
    CA("Brazil","BR"),
    CA("Venezuela","VE"),
    CA("Chile","CL"),
    CA("French Guiana","GF"),
    CA("Colombia","CO"),
    CA("Peru","PE"),
    CA("Algeria","DZ"),
    CA("Liberia","LR"),
    CA("Senegal","SN"),
    CA("Angola","AO"),
    CA("Libya","LY"),
    CA("Seychelles","SC"),
    CA("Benin","BJ"),
    CA("Madagascar","MG"),
    CA("Sierra Leone","SL"),
    CA("Cameroon","CM"),
    CA("Mauritania","MR"),
    CA("Somalia","SO"),
    CA("Mauritius","MU"),
    CA("South Africa","ZA"),
    CA("Côte d'Ivoire","CI"),
    CA("Morocco","MA"),
    CA("Egypt","EG"),
    CA("Mozambique","MZ"),
    CA("Tanzania","TZ"),
    CA("Gabon","GA"),
    CA("Namibia","NA"),
    CA("Togo","TG"),
    CA("Gambia","GM"),
    CA("Nigeria","NG"),
    CA("Tunisia","TN"),
    CA("Ghana","GH"),
    CA("Republic of the Congo","CG"),
    CA("Western Sahara","EH"),
    CA("Kenya","KE"),
    CA("Bangladesh","BD"),
    CA("Kazakhstan","KZ"),
    CA("Sri Lanka","LK"),
    CA("Brunei Darussalam","BN"),
    CA("Kuwait","KW"),
    CA("Taiwan","TW"),
    CA("Cambodia","KH"),
    CA("Lebanon","LB"),
    CA("Thailand","TH"),
    CA("China","CN"),
    CA("Malaysia","MY"),
    CA("Christmas Island","CX"),
    CA("Maldives","MV"),
    CA("United Arab Emirates","AE"),
    CA("India","IN"),
    CA("Oman","OM"),
    CA("Vietnam","VN"),
    CA("Indonesia","ID"),
    CA("Pakistan","PK"),
    CA("Yemen","YE"),
    CA("Israel","IL"),
    CA("Philippines","PH"),
    CA("Japan","JP"),
    CA("South Korea","KR")
];

pub const CENTROIDS : [Centroid;9] = [
    Centroid("CARIBBEAN", Coordinate(-63.313,13.997)),
    Centroid("LAKE", Coordinate(-87.138,37.874)),
    Centroid("AF", Coordinate(46.65,-0.819)),
    Centroid("GolfMexico", Coordinate(-121.531,37.965)),
    Centroid("UsaEst", Coordinate(-75.284,36.259)),
    Centroid("UsaWest", Coordinate(177.513,53.045)),
    Centroid("UE", Coordinate(8.825,41.981)),
    Centroid("TH", Coordinate(-157.467,21.199)),
    Centroid("UsaNorth", Coordinate(-154.872,59.45))
];
