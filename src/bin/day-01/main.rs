const INPUT: &'static str = "818275977931166178424892653779931342156567268946849597948944469863818\
24811432752482413692448689179473928166874161681861461322258513274238616868751793943291175384681799\
74735556938213169184734744597887149176657943367536288362311595787348134856872472732889262169769925\
16314415836985611354682821892793983922755395577592859959966574329787693934242233159947846757279523\
93921784419434659949485845958279832679951257136529467397895592841695512721162423414349754672934868\
78443178642438592386653267844143496189858322592247618573713891336357118194769698545841235895661634\
91796442167815899539788237118339218699137497532932492226948892362554937381497389469981346971998271\
64436294483988395396769866542731459243895818169763959463114299115632725741318662192336963246691883\
69512775194216952649869422617812564123777112458253794129788761342673847936947567322467997394647212\
15446477972737883445615664755923441441781128933369585655925615257548499628878242122434979197969569\
97196137936775649988453743383921783572826379843187465431713795517556525355573596837611574964152795\
79356914879651612118534767477589828548113674226563218368393268189766681915258847632944653661513493\
47633968321457954152621175837754723675485348339261288195865348545793575843874731785852718281311481\
21751583482218547798234227193715547943267381562914466414453822176899273349885693425551887538167234\
25218194999398359198271663187158491617157754279814852334672225867643927836992734522287286671754885\
52924399518855743923659815483988899924199449721321589476864161778841352853573584489497263216627369\
84145516547695448371511212746531135341134613267156156844462682845368718338521597531985871414497517\
45163561172459936965219415891683945742877852336852842943575481564875381754621762681628527469966339\
77948755296869616778577327951858348313582783675149343562362974553976147259225311183729415381527435\
92622478118198711145444737189464535979722949345844352254938676984574255764434955464153848825258126\
73416357617156743817757788683749884514636243321233615765184112344386811718649239168969878367341292\
95354684962897616358722633724198278552339794629939574841672355699222747886785616814449297817352118\
452284785694551841431869545321438468118";

mod lib;
mod traits;

fn main() {
    let result = lib::parse(INPUT);
    println!("Day 01 CAPTCHA result: {}", result);
}