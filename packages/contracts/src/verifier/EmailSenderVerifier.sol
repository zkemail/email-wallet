// SPDX-License-Identifier: GPL-3.0
/*
    Copyright 2021 0KIMS association.

    This file is generated with [snarkJS](https://github.com/iden3/snarkjs).

    snarkJS is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    snarkJS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with snarkJS. If not, see <https://www.gnu.org/licenses/>.
*/

pragma solidity >=0.7.0 <0.9.0;

contract EmailSenderVerifier {
    // Scalar field size
    uint256 constant r = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1 = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2 = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1 = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2 = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 17631913035751363978232012155353767400915020892150684746636420873989383177848;
    uint256 constant deltax2 = 4861494968338409945389826357230570476235718376554414903407482070601109052397;
    uint256 constant deltay1 = 20322161387949967206171169440012381173235723865319356493914237280418553411319;
    uint256 constant deltay2 = 10308591089474590030460433245432977736953947353591145608625029925921712626672;

    uint256 constant IC0x = 2951730585493038522304782150300787397234117234539001556339459475197825421905;
    uint256 constant IC0y = 15760796037573112715079424938836029805611101516338034374741798888119580824846;

    uint256 constant IC1x = 5425859381214848201022225361152352002095946127977826818834763369188796422228;
    uint256 constant IC1y = 21183882257092994928089003438761462883651405030562533573779859312555318999916;

    uint256 constant IC2x = 8592575914727650126244501071109028502247506793505637071411662456818669579184;
    uint256 constant IC2y = 6927625424252160595087924894777172528819640362040626802294632629680068685288;

    uint256 constant IC3x = 2347265459651182678852778801389253696475873522097502082249248536655352564101;
    uint256 constant IC3y = 1584757987794130932232217837329052140097732850454099899547993199611357989764;

    uint256 constant IC4x = 13354798249941344123371878458041490798936908336085050466611071141372192763781;
    uint256 constant IC4y = 12738760222627958454175236031467630003360955331835790219047873104960107721974;

    uint256 constant IC5x = 5893027989980705097465187954034034590216129835633860413874308062328096103594;
    uint256 constant IC5y = 4274695006665656676944775351089772676576668525648016382372716718810615574470;

    uint256 constant IC6x = 2317015710383089423228706263943662214351077501706550382558542806575721541630;
    uint256 constant IC6y = 17576202269717253174122594954421429866213143465956682113364859186095454798733;

    uint256 constant IC7x = 7348261679500798375253087387668930627725407004061317600386111217942965757227;
    uint256 constant IC7y = 12461605286578829343368597354622429798469574557519287214910418696129204139879;

    uint256 constant IC8x = 19594213015411120068613897131842486917838112831027767666567719724080130995057;
    uint256 constant IC8y = 69418783743923849800083670172281491156336601061935718978660559427716704493;

    uint256 constant IC9x = 11946842567748266671570470276165895980702472898540221416965258943160078347562;
    uint256 constant IC9y = 19176033123184764644116177373265021154100559623393677053237703891054362954552;

    uint256 constant IC10x = 550213067855880337165231469561114772633577055852269316162197562883954342421;
    uint256 constant IC10y = 12216142491907415775181124389828468849269645912646647787230365748817908312494;

    uint256 constant IC11x = 12741126375984161571766414477766701650549248695151536512695690568496131882237;
    uint256 constant IC11y = 13652274043195082277391851506987284224931083129476082675554435021804633068805;

    uint256 constant IC12x = 4077364255150137485083425201549939007330217312708974382208477321764096922423;
    uint256 constant IC12y = 8918954888204395726870566201756614322137874949935989796480116393728345042318;

    uint256 constant IC13x = 6545527858558978737903748987936575038345779614138000872424362120244886075686;
    uint256 constant IC13y = 11446785091727466380749819982775385546533845200500472378818202704967847985387;

    uint256 constant IC14x = 6889292862665640586742905617134065117082105284015198270490160845783049073748;
    uint256 constant IC14y = 18801741285377282930403591562312377642971280047645569753356512518069695058317;

    uint256 constant IC15x = 7200341838007187822468144179380978381586857381166577947469110921075751055991;
    uint256 constant IC15y = 18144575521452718130279249689078766581589233112603666857014566928598058369093;

    uint256 constant IC16x = 8704567685708510664011980847047121042904509185476652921363610815903515940034;
    uint256 constant IC16y = 10191086260701445295696446399776632042296488558973194291606527083517650955793;

    uint256 constant IC17x = 12908126112193795726781348719356257732397876739074805002350118443534620222242;
    uint256 constant IC17y = 16966956207235061724337484252576773251452520061079887408332716390604813748772;

    uint256 constant IC18x = 11771933638015684197093637270872869547792284922606572458697474377226524803849;
    uint256 constant IC18y = 6744657219717668460228129687161759161594656232819398006045104034039778081687;

    uint256 constant IC19x = 19205646781753611891871569900497553971166728977185605496401032475126867115529;
    uint256 constant IC19y = 17417369839628191307498421995963106745973321966600091165706625042572076562117;

    uint256 constant IC20x = 6260227910182869661536791649264113704124706898465455352967626443768004691072;
    uint256 constant IC20y = 6484219071422366176279610164655129694470566576611597421879664587392491405184;

    uint256 constant IC21x = 8314779303014199665797386787766877657579866718752342823089282807205728211448;
    uint256 constant IC21y = 12101210445112244037328150587189372030039522393715807915654774747752428365887;

    uint256 constant IC22x = 17701865990621674193574126089001364110464351752183886622475706251049755924326;
    uint256 constant IC22y = 2979309835543399009405903436625773568617899930456602401411433006254723934455;

    uint256 constant IC23x = 20959501839876893681915823041324114035141045146050561981650531297798400332895;
    uint256 constant IC23y = 10912042538615380836610907874826508938676824907210019728194835791473904213702;

    uint256 constant IC24x = 4914336065016228575539437764764146556366834719147126628907103901734510458350;
    uint256 constant IC24y = 16171607636624760093504335339407692213447366321522756659611577784091397351336;

    uint256 constant IC25x = 16064094819511771329924932164308588208041266457965286450281626543105205495605;
    uint256 constant IC25y = 20892251581615434248379465014871100696056145223710139324425715403795811678496;

    uint256 constant IC26x = 17839106959880683359682441521467381553757769975983751120720860742534255203345;
    uint256 constant IC26y = 366142905986213434214740801854075708482061934041650936396866294223518807895;

    uint256 constant IC27x = 6515963667574072747594182689660303457418171861222431547272449407994420956521;
    uint256 constant IC27y = 13255670524365014001423808942550526095927928716212994731907064838635888662662;

    uint256 constant IC28x = 1523598910082882016480901831202280596946861270378227269599333881036196280757;
    uint256 constant IC28y = 713657011709961508951402488452763591895513323473323899115135196175170373633;

    uint256 constant IC29x = 4847432377084236974655423802850948677230313262840755829215709338165614661624;
    uint256 constant IC29y = 13169255662647770373482919490436715030784059416631998145948697053669294704932;

    uint256 constant IC30x = 11871524670718110186107971181424673400262556739137716979496296350989124334727;
    uint256 constant IC30y = 17424993173189466273567584522369269232074004486435156500215934292894717342749;

    uint256 constant IC31x = 16699983454064597523428627892143403964581925023639834008358531657860841408396;
    uint256 constant IC31y = 12878543938087340723980913781867155707674360946337564368020944189358190250709;

    uint256 constant IC32x = 12542233660582317567049233681647369899162808392844616769775607307283356844463;
    uint256 constant IC32y = 36822361922815328695601659584046198516969163310678126639565897401836170418;

    uint256 constant IC33x = 2340149794740836286840625850093347404500086828136153310003424292084026833115;
    uint256 constant IC33y = 17309354344134462344666058975985037583829746299853511147702523418822691023248;

    uint256 constant IC34x = 10713849480395287194643616800569242867988520656219643550911535073332533414836;
    uint256 constant IC34y = 1446706010987119819678146310954413811697015293103174421204990798595275832651;

    uint256 constant IC35x = 13945607490491510189888125370882265245829491178075587949891478178001131864269;
    uint256 constant IC35y = 3354922379724252288909998983101982349642120429831197568080292752566814725801;

    uint256 constant IC36x = 20920471962125432031464929826326857047551066992272290697632383991470612338589;
    uint256 constant IC36y = 14877331017637392628187332584239251384991028938015877237980323902455607634066;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC,
        uint[36] calldata _pubSignals
    ) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, r)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x

                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))

                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))

                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))

                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))

                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))

                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))

                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))

                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))

                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))

                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))

                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))

                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))

                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))

                g1_mulAccC(_pVk, IC14x, IC14y, calldataload(add(pubSignals, 416)))

                g1_mulAccC(_pVk, IC15x, IC15y, calldataload(add(pubSignals, 448)))

                g1_mulAccC(_pVk, IC16x, IC16y, calldataload(add(pubSignals, 480)))

                g1_mulAccC(_pVk, IC17x, IC17y, calldataload(add(pubSignals, 512)))

                g1_mulAccC(_pVk, IC18x, IC18y, calldataload(add(pubSignals, 544)))

                g1_mulAccC(_pVk, IC19x, IC19y, calldataload(add(pubSignals, 576)))

                g1_mulAccC(_pVk, IC20x, IC20y, calldataload(add(pubSignals, 608)))

                g1_mulAccC(_pVk, IC21x, IC21y, calldataload(add(pubSignals, 640)))

                g1_mulAccC(_pVk, IC22x, IC22y, calldataload(add(pubSignals, 672)))

                g1_mulAccC(_pVk, IC23x, IC23y, calldataload(add(pubSignals, 704)))

                g1_mulAccC(_pVk, IC24x, IC24y, calldataload(add(pubSignals, 736)))

                g1_mulAccC(_pVk, IC25x, IC25y, calldataload(add(pubSignals, 768)))

                g1_mulAccC(_pVk, IC26x, IC26y, calldataload(add(pubSignals, 800)))

                g1_mulAccC(_pVk, IC27x, IC27y, calldataload(add(pubSignals, 832)))

                g1_mulAccC(_pVk, IC28x, IC28y, calldataload(add(pubSignals, 864)))

                g1_mulAccC(_pVk, IC29x, IC29y, calldataload(add(pubSignals, 896)))

                g1_mulAccC(_pVk, IC30x, IC30y, calldataload(add(pubSignals, 928)))

                g1_mulAccC(_pVk, IC31x, IC31y, calldataload(add(pubSignals, 960)))

                g1_mulAccC(_pVk, IC32x, IC32y, calldataload(add(pubSignals, 992)))

                g1_mulAccC(_pVk, IC33x, IC33y, calldataload(add(pubSignals, 1024)))

                g1_mulAccC(_pVk, IC34x, IC34y, calldataload(add(pubSignals, 1056)))

                g1_mulAccC(_pVk, IC35x, IC35y, calldataload(add(pubSignals, 1088)))

                g1_mulAccC(_pVk, IC36x, IC36y, calldataload(add(pubSignals, 1120)))

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))

                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)

                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F

            checkField(calldataload(add(_pubSignals, 0)))

            checkField(calldataload(add(_pubSignals, 32)))

            checkField(calldataload(add(_pubSignals, 64)))

            checkField(calldataload(add(_pubSignals, 96)))

            checkField(calldataload(add(_pubSignals, 128)))

            checkField(calldataload(add(_pubSignals, 160)))

            checkField(calldataload(add(_pubSignals, 192)))

            checkField(calldataload(add(_pubSignals, 224)))

            checkField(calldataload(add(_pubSignals, 256)))

            checkField(calldataload(add(_pubSignals, 288)))

            checkField(calldataload(add(_pubSignals, 320)))

            checkField(calldataload(add(_pubSignals, 352)))

            checkField(calldataload(add(_pubSignals, 384)))

            checkField(calldataload(add(_pubSignals, 416)))

            checkField(calldataload(add(_pubSignals, 448)))

            checkField(calldataload(add(_pubSignals, 480)))

            checkField(calldataload(add(_pubSignals, 512)))

            checkField(calldataload(add(_pubSignals, 544)))

            checkField(calldataload(add(_pubSignals, 576)))

            checkField(calldataload(add(_pubSignals, 608)))

            checkField(calldataload(add(_pubSignals, 640)))

            checkField(calldataload(add(_pubSignals, 672)))

            checkField(calldataload(add(_pubSignals, 704)))

            checkField(calldataload(add(_pubSignals, 736)))

            checkField(calldataload(add(_pubSignals, 768)))

            checkField(calldataload(add(_pubSignals, 800)))

            checkField(calldataload(add(_pubSignals, 832)))

            checkField(calldataload(add(_pubSignals, 864)))

            checkField(calldataload(add(_pubSignals, 896)))

            checkField(calldataload(add(_pubSignals, 928)))

            checkField(calldataload(add(_pubSignals, 960)))

            checkField(calldataload(add(_pubSignals, 992)))

            checkField(calldataload(add(_pubSignals, 1024)))

            checkField(calldataload(add(_pubSignals, 1056)))

            checkField(calldataload(add(_pubSignals, 1088)))

            checkField(calldataload(add(_pubSignals, 1120)))

            checkField(calldataload(add(_pubSignals, 1152)))

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
