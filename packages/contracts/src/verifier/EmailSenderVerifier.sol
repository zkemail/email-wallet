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
    uint256 constant deltax1 = 6310705631016798522239846231975645999283209332706921797762090298897452543565;
    uint256 constant deltax2 = 8043235449306610275940661611843822444931681380816964495431887694600626965124;
    uint256 constant deltay1 = 3079430518131556256306481264990634155039910530915437287975212437439286306919;
    uint256 constant deltay2 = 18589728556676403934523866231401773984467676559521991485861435940541539709656;

    uint256 constant IC0x = 8762606935184873852577051184493725125807205767954397600981962523745255796999;
    uint256 constant IC0y = 16178777099787661073291723670831255501033792416920588821034847534987608542478;

    uint256 constant IC1x = 8709887609923114514901941956754341991777746679126808348406380268334287197999;
    uint256 constant IC1y = 435605939966840424036666267967697854938013940780626245348738424380495995479;

    uint256 constant IC2x = 13026547111862076540548876769907451683963598031799528702186587651581559229927;
    uint256 constant IC2y = 8360951661332142658207229924905182574689772829541054037026426062659029211961;

    uint256 constant IC3x = 19577862023126676171801128222281934324778157229879191890421220560016775751692;
    uint256 constant IC3y = 21671075590024172931845235199967823131387951004067405804095145883453375538176;

    uint256 constant IC4x = 10925368485505485507560159150135993115934046608410623239937702134817859914690;
    uint256 constant IC4y = 13105510677241172902053558422519879674058378544322652892895900065979274333541;

    uint256 constant IC5x = 11782501784099355835932530735340331599346933682387436344449438347225690681315;
    uint256 constant IC5y = 7022573991659653867953866576253798543982628399611233461538153162206382889061;

    uint256 constant IC6x = 14708471204059661143275850868727628291496460180929697177717221031606871410687;
    uint256 constant IC6y = 11275490050368943225940436418854615445360592705375407437768668116290713170154;

    uint256 constant IC7x = 20042222910819509964849006624912723711623731130865287168867044386510544176398;
    uint256 constant IC7y = 12073844497788955455280668284096377271656425538345203788027054392812012409297;

    uint256 constant IC8x = 4828973460816399296245183761311599754664283456079677519665298540989202658457;
    uint256 constant IC8y = 5072250312980534251738886149426084195184548844956101524392018946469571777417;

    uint256 constant IC9x = 13484548930722047475154072236217718480444768440063355568294460995888599896448;
    uint256 constant IC9y = 3889969169489391303075011741998805979460916386709010933366126634055736198659;

    uint256 constant IC10x = 16910917228637910969014941123277364305254141091147903023697262093542055432684;
    uint256 constant IC10y = 737381319144192281466353001029596346306971585618290787574749692945490556168;

    uint256 constant IC11x = 4553747434016215152929679019339425557856041239193674623704825687892501381318;
    uint256 constant IC11y = 18547975621311442259132134996515878894871124968199227638273697455130976565097;

    uint256 constant IC12x = 16616794786079121849784373842862464823689299806688644540103585082402447825485;
    uint256 constant IC12y = 16409700877743629882844953251117070017765381198639854513344980205628494066999;

    uint256 constant IC13x = 3160568290729339154671371784554789777193549652602462102331599112532961562149;
    uint256 constant IC13y = 18409932075138696928564081352626600726629151331357019521478098007988605259050;

    uint256 constant IC14x = 17844007657507541810633171830492687209986377954329245090482164520266605987459;
    uint256 constant IC14y = 3233341659052943800778982317050362122190629550491459613495929741937758724155;

    uint256 constant IC15x = 12272622112634363434410192465943462657159621366941669748480380259677225473967;
    uint256 constant IC15y = 11228945645506471416784588250959047105989812798793178706745381785859467373373;

    uint256 constant IC16x = 20285087019607460948325984932816849165923748857200589840526079350502279778465;
    uint256 constant IC16y = 417822303548123776230729476717667411014225211782223117376138110044583751315;

    uint256 constant IC17x = 2443022809919983245321721945761284183080957394155588400848769538376841341588;
    uint256 constant IC17y = 14010085600973514446473235191279898791040407444095140687930937182543412421679;

    uint256 constant IC18x = 854637621023914921422417108042570705260158741368757676602552738353838563694;
    uint256 constant IC18y = 19402838419069105547473348553276816519895124192002960230840527808891499552036;

    uint256 constant IC19x = 18276801647661569083534335101083426843451159862475226066316318760455010173118;
    uint256 constant IC19y = 5928529608121718571166735263463842793029881590965648752437166988030373259866;

    uint256 constant IC20x = 2581663819523746519176136751940864856209086554067561055492296308126497326111;
    uint256 constant IC20y = 1896930729303562291514117380287837215849561796622024408915829205364372185392;

    uint256 constant IC21x = 15445024538221295111267883818491678754337050338141526228332661281178191672500;
    uint256 constant IC21y = 20970929212982523092442662526019562844720090905262980658365916847058332549503;

    uint256 constant IC22x = 19629037075066098557442408883476286109013407617388277873149174344619477486955;
    uint256 constant IC22y = 8913513923471703424038922675203257832279024377686636148688746330252038572788;

    uint256 constant IC23x = 13103666726066712695576408166615269143575596529133503685160982997687203517603;
    uint256 constant IC23y = 1898627439536220755392968541641907792453128089727432905517364755744230279887;

    uint256 constant IC24x = 21328865403311576366229599921376438153192988725611989599794928559848996495343;
    uint256 constant IC24y = 15962161233886041230767711509668388884646876136509073996720320352568037027181;

    uint256 constant IC25x = 12112999482942119099676040129339791967534185365532646931559178476231914766379;
    uint256 constant IC25y = 18506821422820524526479430293428799312495553362994434931455008749408721603074;

    uint256 constant IC26x = 18238974048220876930496054867626074994507439191724669345574379398714427991309;
    uint256 constant IC26y = 10191839782438144440371407385443524023306909368098782556028230274748760812208;

    uint256 constant IC27x = 16715714550658660250410748242719801315981726036839746533506093746859642844955;
    uint256 constant IC27y = 14990703784516408696050643005969750376110238922957112398320719612636385235823;

    uint256 constant IC28x = 10437000851041423649202029913739924528846355687777751084567949458990925418488;
    uint256 constant IC28y = 17908631397082266575262558891728688760463979284551036780675010220227234515765;

    uint256 constant IC29x = 9010860177185323559849232657184114511074132222083738529889625069107108059544;
    uint256 constant IC29y = 521261091782817341735132992219781102774577704093267764985617313040496302143;

    uint256 constant IC30x = 3219773004970696909770979175914332457645195330399551939151697587509111262139;
    uint256 constant IC30y = 3122162022703781671649405453151509157504406247909271192908968204720824361401;

    uint256 constant IC31x = 13027069095004103181535510179723788098040525982667411607013812634295177778331;
    uint256 constant IC31y = 11107906386093533524176252093246992843853813670979049460748431325833114118330;

    uint256 constant IC32x = 20340972429765067668715752698679354157838587546863215859441279869706221047739;
    uint256 constant IC32y = 1817020359752349398063578517678062667179294162911130738713128498994559418221;

    uint256 constant IC33x = 2275320194591044440479803765920936180608418089900448632532079919532113889785;
    uint256 constant IC33y = 21361049156174725345722054939127222698787547660650241080890322060935357030212;

    uint256 constant IC34x = 21145051645503283182458453694396720459674391457177266596553128939892833763514;
    uint256 constant IC34y = 16670166988272424120857017720213867447147076631461813803098627072294199081943;

    uint256 constant IC35x = 19365482593594262952150464772522978344879714295174689157679079942313511936846;
    uint256 constant IC35y = 9468073211421785593866973470799915426253096441964618356385485883083121929150;

    uint256 constant IC36x = 13772237936678360990016056163460578385042427852108086787898172602107690038190;
    uint256 constant IC36y = 1211095664858130392244567204144372962787229556109342250129039614387141835811;

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
