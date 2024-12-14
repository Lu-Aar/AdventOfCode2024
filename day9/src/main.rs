// const DATA: &str = "2333133121414131402";
const DATA: &str = "1012178791354059884227414319879232325738461439901378387779851518437752468679529168936396621359308269232626508136869324122534475550605864418071101975413156653674439631435948436127194792499553826016238021438471529762338860374729836038996516165317721740664923791198807457192377238450164333113686357276704297236565705645615080554045299918767178949820248292555882574664284312843376235942728171155576629425863732174463601675829051378148121684953384752359431698988158234839652248144654314125763227815886261478734234933563319814792032725829441393522976229528666192415493263040848176129387339170406293868830668819227815199582476114285799405652631824658614545377313359974426487537177689158636705232517193111920352543468325463854598447539447481479706442788223704054433729148410637336509829744876609486565713488449799962235739193395141169826460662355159069931178593629159493428240696877442494247340222346636915957512819085132038865890591520437411805674475048252095458164726845427474985624247426365990661456672865942747143619206275323981696484961020823897718416469375775384994523398184865027477246157333426021861785648282673891625125377992589056365324539151977282611958725614617965662248229985806687266982881938855093459298693359424431857248656550244156401478589414828244517935483120607035901516603162461216637567269266326948112521137058751620813171831720544032648948896322379053615550505442629690247015489773339414904312941752308287643155469873101513845285357095997311638458175581758535292737863818917424112051521831987981498941909015734571491824755063474286323846361752392060675480505457602369548028649460897380839862416192521859717010959755485565526622409663198063624726493926718485616087555436131353603420794337238283622513423915284280402127293290175623389888698836349936139794715370784725421858286368847059354646822953721912659063847587304136684820452991359020723560444387712712837034736328171777309356868236279435698478384183974994158164398314788831525065492912747143403866598429791939862691934143269799765146827089624533339748495016834349963164714810563840904081576718224639371837356930717961133956156028775265561240142791743729982780682871433059721069608415473172763754698599193947145949901829199642819660732560243045866467905276939467124754294517538360291185864498522487976077355395672556101217701536845418371811759298522349378950389040407884374533264450454925993091876816749974306054747639573078377429162240464324968868453238508351898673672017303984352876556076993895962226573760371525258584809333778693278046168846505963278322313680406828578775699765745571646280369514548128304783436625529222174918233654139188437264293542657731941367539533775476849094593269526774682522214775671898248440797374617988165961907389985693298138756842545710865534246672817233295998145665611031631343236492341035106679514844267730568469895372103761784388841435864648685636682615999362623630542175951722446744535289269028728023582543609575735150293946137455754987685426649249132081535327432353214036688876185537669596502565251842864619856898679864102712645534317625917829749052637926982747271656349998322282258260252877286259215115231072171495792456116577622180332124475375736587992234834195805868435877801035146112401440412777477711509289972151484420981189249752835541365117151270472773816899327075539292711050597897278566366298406421999447429569444764511997472954457873683012928620781415672959662242163970497642207252286237759346431915848770412491628598751610594745105175823294747975686415631294919145479464995828944360321141156144252760574220518892544477522644365529712578499217107175592750473911704370683399472559744984956169881215711896576825169784197190437992339517302624589076251365464739495944195537247630484499588676215276215278902142981043159643896545386996945453293870647132898227611268865420774636887696411263469982608234873595607472291885516996791244651442228165831446251120442418205916627354103812687834647329601730985718309429259961447390904766102653111940765695427698997269322872919848387855503550969025722335353262878854976219737226918599757280497198627064188730941791903684481411566412425358881473701434937999704897252365416996374386625386784672691457148588735190953142236775887544466431686364796423171336327349159046225796761433775193849375893950123457718288173745135876649355578634728658625231417740422597837541581768932184325499612146899820963857489322333652307664937433378511207181339544192589393572648680389629643180667475136831594134926434953066901335418865851844315121429386488317764426221553669810108513584843842346994232782214373421617699165466695684785219897940995754288579231337591548605172449514774760201791839281585222835035314026302584318471967110885112533134912522704732369059321398385631464481887214974095841731139671459838347580626740171356243247873092922386859642991346194181749373212447298251981843349347928937451612821416132324577277379425461956249596402366475223629520617844924344318022794715861766989315436110954115187550971647704165163051652248283074958132628121944352414023351866533736184965895817659762754498581791862088334517123074349276399064385862866380575797628034454512734382737680551485811674468352563262392666963257586834856218764271417326989048351551953349603727284645752166476773841277194138855264332124768747241352702428906652242522992325549126457176358760791018607564458032327672245185983746472063274778244955901162981121485641162171509119195513764422464465436964492358249945141671962874535483924228332725565877446845555244299688346384745856386764276677151990256059826122477279409695918769601891828063968784499230358231768850105995433945381030481253711114778026654756705371837920377781869235156525183779224069496530366564242469179868694551758852432917367191482628283646589532894565922154973265536849549287154455333952396828985664307980922228799435683392619630579517348444394033651890875584268665545142795958538265261190711330763777327920403229916772703634605863443289351673126598189764199532649398483761339533509961422126577278422692834068444685393789785537135232496363295786513841863894404698776660506293374521143614181730401377665714571211962149757390514096259124574881117124863642341537395431949521615979402273918095373461217185222274392417606519961414911676798949519073105341926878205066183685467047962999968999242444178479838775162130647194809664611775352189293088682992124463674239155614422954179398477895416240798365241417727187867111545651648915538173454542144867788730195253267462773366173087387657187835989914876847911242454518472834436636186568369795586386552558939260434644169630396434958733645895779725763895243822133547521377586443952919254910707039942927183399242393186229364865649015175067628299891774766230704827121819769471213392167320897377621445762650196152552175937777908251464969386497464610525628829379324056546076413996354636287537272930685857865845757378706682112223523654321337606387737071844938805992988753553761742772597870239258103910941370108198157371571687578293452611299911575566978884604035602842637916557715888170176620134799401539739693812016436040698616992064775375773437297357405823121554593081908568583183731890516866163786874097428421906873719292958188278127637280906167572296265956757715655192238768527359277570555126203058445431337884282759584066441464245981314486439778195380528179879211845836595276845773795722591420958136908230731628315280651733935091297250886959265098566586539890843921866215466818834829543151376944593753499485188826697086258982429722281696448022908541372983181775607332898530546150306860685513453454136093143942573480713651743787366444413441133889774051455986614497703484264031668992454374552716989850998656169032266666955052667790683435385658261870569466857729703194755728381891171222922927301344711571814513139585692995701767623441728621642931197742269659996810607414461043657896289911603639879186854699287631198916766151126779279824221444724120225958381995624580277587909956569886634383132175877343803915827480531692502436234586592323358452268638431847769694211130221374354886564389658677838354423619162429874550199486418182882817948114101730373463961063358361335551644262637651685853373727946989917069641884317467639442802678647426974739182313467259642593952310668828732797659757575619346145443925214833337588438494682870913125726763636448445641687552486147874942402483909862606862626093895820211488364792831659279417893561889773962564708055301851854341381620592440848714991846473937613673874233652870552291574286459056234369154822581483477930383310759282656325112760801537617441706373186083891032431228992711443843937417433198473019282624324345362720551919394539392230756288691776681571756575178072572367911774758928971966805555746093936376223945339266646518473586652184329713134114895213518410751998339135575554848324177792211283834453683369819589913028263883685650135351437967477081331436906887161732663176211284317882575772236414562032456760686318184365668383615673223550668996983141331775341898329719493192987487143986754277825748346453312454445365973078755455904710844046883316344435558341565789844183638026669131132517366198701861802763275383762398518350892594582987774677122765162099406694373256641714436321735981503274486658317786784054907319823945188218451381273586768467322417488289279712639320471066384271408833757629614168537177224439487453984466836143764032806350753885539794743347433867599852139942731569196457507959771745391594896477197771903791643173413168731439773247204916706915463111763793676869317338775781477668936054222678853493863163754111359512978153419817302037735443102872911495157711597327902174144835398132107529354767207483109954461752323570804649109263931299103674794790632790206619275936149627929057508569351821603371771328666827623294266212673451555019308732815889859557944642326712382654176959147483221330483891669066606843172332504525788628958665207334609725183985553280669196421523859326152976445020217286813519914284688078965725963315711954374041656921694461591980326226532458196369801358237621709186261079276583426419649794677339659528192936384043351287472757174817647759301548148832841549674660678013106049186417276796304652597886488498387234492594348467459385145644537835571638915681972746247181154058739286906427236538252415679970437836422332482837919892491995217195829437566291615573757662212717271990868474864176672028293487777082887491477617275887967065404642272135738818751459324490992029578745718081815963618383975920989068403791202166677970362351117710371482965136584255445153989658382593416021888973687217569795944111469589865291146187743255856643559691269980657094636181516438144251777225831914517889604342903278644758527426382150177384931889641872739084698386353812765483736632227089293961606476613082831865643943254510848171201757209586245212439563136080479713827658785683928771457778346593215039152434702923822818128135357128668176215790641616246515875981638075506553293866229322741741116332534430933614737355676582414642922167465893671828532330515025586937709329243328499992227485697987517474508938443223405219327682597130139124642865535081847866751724642131136244577540591674216671682168805259421539996794703642689535693895762342725996951073879835699362449373927150832733727428533284727656444610645950646477768317569774544471805711866792542676362681229829852111335228437741552186694067462083229891299895805499101163727320677853618041838246862527431433632186695024989096914414451235559695847289939171376255461118209731527328421192342178912634896949309739301123396064648674851357408846127545972014892682816396465592699860657189927138127024574182167035661316843650679585815880777942859614152669615151549880155758606062145436336843825673378424424648194087633920127967618443932862522523327976974585486323591831903958598232586178703439778541483864705717704415976794915329652076684932201851868126171581935721485128986155174933379496441070897387266567387876324761752216452940564456538783124742169728202681233139535465903212215782781042241835119269155840976739118310808918122667234782957319322127333335329778394931454139995310998948684191425383533241484370201333244441304556116889915792739196487738214656936774896460148194952523875421692965764697392217342359856127409688126511968894435488399676127722697485568684799588966517626891936559187059104918811926798294665055657530441599795318287872229147801296529661232269327858554113121825745983193758159777315330701951905066789826833358676211501441813214657091645686447458537435722938669486761515878797113911986479632077626855682365912342431594511132534470552048143387912567739838561341217845126883263123153243399532576245448273181722521070753370931114772866109751853469205327243532872848436546307877184153896937294680511868294040225665165391607521887824676034922681354255127379218683188492687060287080791676306783653017249752758845399921888616644151739426302993307378129383208587585916603897509480519772146480615615428779683784981913365227487999152143726825965968208578933785325832518513129624566463592647624463629458943489546365758596489464793324757642213341767824179243899840378232535599717860423165658974331326736019802472183037616267508348434844128674614031865366353883986739644064541065377181386871698128484221678519373257925562365149308490923170838739152370515229537037609530592011732751896934532619902418804664834955622450905578387972181537404151138352272837836296328035933322875958376242711790688022483134825392936250885328283585747414935170827873496053206471974018725916218638196624439494531349911739512290704176331424123177278783914755462231154197658541352358205897686224744013259544982257956979496557218795842231754941961873199112999093938147818179663292362097734938815519653389748438601287775578603296865277441939732745771141602950577571468115872383952070509344977357813468269763714029383698335063302411229085118018923310383295407985991177398263126366436677927183433779608827582544523289822761357331302471395372824854512683687436866237267090781420593539142562246853811119747661863953921142644718194563384066128779251955528813915580777812373636906830419090116214887757478628643364872774262796329494739567201034136948223543274676641574884955553175276924755382656381811665681196442927346779198456455314906221933981933442317842877219838431885247439226328220854216415210205156394621606035761327307986755997905574602697777349222799362073208520262836695515429762634981159655523417921053812417952774274110569271796373858295187584394055612872724459466044885234678739471523752634447622426753557947968042259585423764928978201929795055639494262011454840107525122598678280856722264298608516707011542115578365113795921721641627305058955247164947852226578387764985222554121543734248843465916154632011875865219736591491112428788126437893762045954580763890424628521297238461836963396147174021523488501914564117464233202913955653678583202470558391338164845345685738809645305825632087722836336586162789566293382359885478863452885646969653503197356923539525157188107172358743585963383011512175511447701970775058133342427574202149465332617985367710689629552210416612609726645687885519224221319781809377802868136911719232742556446780962887262941931859432353785660933172939234552391696748271178516031625659382263116177509198559286722666963546211561619072755095297553434830215423596685173272367293374748143430804215775232213452145690603850402048452885399449628994829868181874636320215894523996115040151559538899606277284693179867166289306458548289463712979335701896245262122410231250795423937039694540616066206479549666659835204497482233545755179691324675909569278737471390628741979235473491755074501559909468572988816050792472803118686867855783408837946341718113535188341766562957498770556115938352111667962294163068347078552213476545403362615395857667357176822331825934905126965233225177185450114239971472678222281534783474793515376387383955121748958247854384464895225657809694388760181843482875516364344445146947136381254815497572661632725118407889489141984889488867675856747047968511697322506184654467359986977027112112688337948981365624428722891936211952153243564624986953185848197614614039237349853262812770416470965850515936728189168741478419419249401586156057173267473091195133222443238040655058266474538336962915235781418773498384235387419037295355581596889255485215595322601825529831335074908429249578867798337917213268256879346023296890139833516313535738445164101537329876302774336578364647866543553541519027253472703511801649636540142671746288462067315372948833182781111427176342705647831766327289532566193190357556907199953943686579202577242149476742739176783379773759349743369595853617592283763479695696266418301358198078611343319932404983153282504473253116349351269617448995691676901179461577367278188816326215377229711096678887266888264325578229121718224281103781475880537638789135733320282751239090906672213674602159194777137411718496611333106941726778545827559098464114301329508494394212397822706464644866349645703060108133262970857580667820534368595983153551702357544224561747465716533362529286377376318112113598872335787796586748686816622225528040668773544543377878944264665054172588828414687036153091936610607555357719598945303778161261575589305737569567495049457499304992621828216642408532795178982771297426365455889810472281328864386747277287756676253368134137874974236437122136168289835370287243786510165998814239564642947892587539223427389043847284722311553530276359782567763277617076263385514925219188283824757760754791879613851032394024445688782465891637713838798519468844939491713611616591689964995934545064556066677745743894313444437965738171498690828042496881731017913070356498827824115972767932179179933113617940649313764695308769441822462215323555627929635568236021942623431146772318717121218275883411411059968147438547691444927227255521387645837319505448786943911120336335192734673746964373135381372773271541473114161440353341189094796549142695242724875513145279634081462231869875329950617621661174614470594670418131544867814292325354411396225062152332803764625767587222764689636349351017206921255139504232169787873187459512438841292413679731705862362484677476785985805510214783538343286516852627181517538991766354238013941068265839886784838110191567662950886777591938441540947637792099671923458776926781277884341948868762852561995471194584346826173464455263828912533249213552105453895037675444931640475081581038634722361571323588308240395211625199859413107671153430849630855459747177403857525763448477319674546822976863338844848348768455299347479835312965268329246355693054728896894067208420886059346466931610321658148437401078116866683146641432633534538364168036276981202133686241568530319097143124374634866988478486935657572456867123801692593882255471513861223536661253503323302577143347118968618023517497482726757810503498491986728221335511703692256282212718181192145085251450996627603278561858387616535170885527239473147981356759547414287368495870723573302966765258499463371128804243668354739226977242708318524161564654504048408689765016915476281711423684247193711247765843665592865612539168439275462927206059707458965496674377468275616799927471461793461326255329664671438627763010523932614954394591728843924089537415212370539256842052858551341240715935944420588785366447326850564722474715171827151163588387204766493869789912254963107781176892497635337321488912168696215367316162877438686687597629224684642144248242337461422554258548999672139491887581511560794775154983788938684061137085392584253435595773441926958337663518582689429368193647134539333945125674311063884375681263632360978740519672999778176998314195566070416536138552856750396572377424655524882973491954245549122462645495954733592532327086241879338649101775165678133395214336879161953112704868946333235955703246455627873768518557865430594838759229251210918110575225362977575475106466551728274261576522373359589383262857876394877214823972556983482687103390403911412645389899839150444299202354356579437140137881266485335821883975755777789350944976844243435673914761918739558079611443371433528124194635309396698990523831737867829993749564655598979748668736452039516491939228441882735628776515176320281622236417476343495190494";

fn create_blocks(data: &mut String) {
    let mut temp_data = String::new();

    let mut address: u32 = 0;
    // let mut is_data: bool = true;

    for ii in 0..data.len() {
        let data: u8 = data.chars().nth(ii).unwrap().to_digit(10).unwrap() as u8;

        for _ in 1..=data {
            if ii % 2 == 0 {
                temp_data.push_str(&address.to_string());
            } else {
                temp_data.push('.');
            }
        }

        if ii % 2 == 0 {
            address += 1;
        }
        // is_data = !is_data;
    }
    *data = temp_data;
}

fn compacting_data(data: &mut String) {
    let mut temp_data: Vec<char> = data.chars().collect();

    for ii in 0..data.len() {
        println!("compacting data loop: {} of {}", ii, data.len());
        if temp_data[ii] == '.' {
            for jj in (0..data.len()).rev() {
                if ii == jj {
                    break;
                }
                if temp_data[jj] != '.' {
                    temp_data[ii] = temp_data[jj];
                    temp_data[jj] = '.';
                    break;
                }
            }
        } else {
            temp_data[ii] = data.chars().nth(ii).unwrap();
        }
    }
    for c in &temp_data {
        print!("{}", c);
    }
    println!();
    *data = temp_data.into_iter().collect();
}

fn calculate_checkum(data: &mut String) -> u128 {
    let mut checksum: u128 = 0;
    for ii in 0..data.len() as u128 {
        if data.chars().nth(ii as usize).unwrap() != '.' {
            checksum += ii * data.chars().nth(ii as usize).unwrap().to_digit(10).unwrap() as u128
        } else {
            break;
        }
    }
    return checksum;
}

fn main() {
    let temp_data: &mut String = &mut DATA.to_string();
    println!("data: {}", temp_data);
    create_blocks(temp_data);
    println!("blocks: {}", temp_data);
    compacting_data(temp_data);
    println!("compacted data: {}", temp_data);
    let checksum: u128 = calculate_checkum(temp_data);
    println!("checksum: {}", checksum);
}
