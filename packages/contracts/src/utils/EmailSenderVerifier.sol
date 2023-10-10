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
    uint256 constant deltax1 = 6234386830845997071225456987984938716282941068398289666409907542235335903650;
    uint256 constant deltax2 = 12453658547413995564882898559248385722009891585889750415501276262056392630287;
    uint256 constant deltay1 = 3066002216483936554065027266177333430409880463407889443083330802182124877232;
    uint256 constant deltay2 = 20540074614654446126879278444495563413518725921832505455148995511134209885530;

    uint256 constant IC0x = 16942653915586966122124385847463105945963788827852157050728634471112715243626;
    uint256 constant IC0y = 317798272264132251687933196900449582990210791683477878157210062620208852627;

    uint256 constant IC1x = 16356551068657309890663847415572443666271364639567022588001798654088986793260;
    uint256 constant IC1y = 2786730024306044676590519265831563122591020354507125306230538743348069547846;

    uint256 constant IC2x = 19535266454140026052831905499440432251574892760832620522668944748223309872399;
    uint256 constant IC2y = 5502234044793063239426504824131410399900741023884507256789180464497273393990;

    uint256 constant IC3x = 9346932414281224906891474373762055055281367356977333666446867963128186161952;
    uint256 constant IC3y = 10388403622520008338076662146377449849945016341415058680250966089336652958456;

    uint256 constant IC4x = 14950258128912160312827785199036026884218738459132903562513908498337686648841;
    uint256 constant IC4y = 381284693351692747211495820889761107257993578677453693046140260471749029831;

    uint256 constant IC5x = 10858198304504925583161007516926318282646298277299013786202031193088645751917;
    uint256 constant IC5y = 10608646725003713306086572867112596802192264053491602855309539483412603722718;

    uint256 constant IC6x = 4178393559415913151128696226687323697710403799871856406768951750008630049595;
    uint256 constant IC6y = 14734248864810435099196007781313287263087503433309023791264243065137880199774;

    uint256 constant IC7x = 6700020644374091509349456488086946859577730454184622041252715612675604266026;
    uint256 constant IC7y = 2751658758185284023895657894622503780508788628575941151061436098713062620708;

    uint256 constant IC8x = 21553446581248987339877247429417356325148527088028949219276091419463558047019;
    uint256 constant IC8y = 3078573079669571315483773352647446503678601718764489680146581384798912667208;

    uint256 constant IC9x = 9113455368687800926597882739851218408146575488101984154941659568450272167780;
    uint256 constant IC9y = 8982346142355046312535654843277122628708620673246388518597119318250916492063;

    uint256 constant IC10x = 17048283493170703816004324250025180952138826307094989300113422857083369820019;
    uint256 constant IC10y = 3648255262397338993116193326732848414619817643441731739105670338876419754223;

    uint256 constant IC11x = 15688573196374666083624213005365408080390398105352356727388582161499826154830;
    uint256 constant IC11y = 3962572780451409433816812681067228257247450705891511274102514431811085674744;

    uint256 constant IC12x = 4307249632426026725621497989782264228802950366555474840507170988853613200109;
    uint256 constant IC12y = 7068005855215926799946738737185693242569676388395622157990703321686206374962;

    uint256 constant IC13x = 2130102824149748172668722940112907399043814254222209348422330642729667262862;
    uint256 constant IC13y = 4526683078612541851074020149710701760494134256436525651287833017868438927152;

    uint256 constant IC14x = 11630975744436112086081961901819921289072066978478596825566142393454940173540;
    uint256 constant IC14y = 16194082209907894267799299544548945988148257991312503951261860364374881215904;

    uint256 constant IC15x = 11428536468660348684668307115872880769096660430958541507134372039325790432138;
    uint256 constant IC15y = 13201661434252643346868295616870603508973404651023632623049511640127116595505;

    uint256 constant IC16x = 20848466392019134975034389779561825423497974274255849112270964099931320437596;
    uint256 constant IC16y = 1546996386197566419085400980086634834518373722452607079046010619844651997503;

    uint256 constant IC17x = 14736620243584336627483277929961833129894021092292809062855553908047619968475;
    uint256 constant IC17y = 5415629375291205929464731912753889992320799172869492399232276465529968939751;

    uint256 constant IC18x = 13196560888750234471136773234054705434029414305392036767636865792648164466627;
    uint256 constant IC18y = 19690577091412032216508184887446907275959045289616501196766641232869547762921;

    uint256 constant IC19x = 20009260569402629153567042984029658492442306810727844735382566111046120913690;
    uint256 constant IC19y = 3877397665577345493784695744415983929891201356322119667287157631515582664337;

    uint256 constant IC20x = 15359137748614582182374289702565770313977370799824173363146878128022612828926;
    uint256 constant IC20y = 10152335709674018598266382268087091973675636964506086686825044039760380455777;

    uint256 constant IC21x = 15094309384664460631291542397134168343430531763486756229641137601757551144149;
    uint256 constant IC21y = 21501502517516836080338730306369765503390099129731798042765308512844694575219;

    uint256 constant IC22x = 5454929656529793907539197262599709414792427444104121438019604732900539009401;
    uint256 constant IC22y = 16669371821084307029622477915948893363222231105293764480362424504709957435614;

    uint256 constant IC23x = 3887575991561369185843859996706412586449993938894992471022908985348421400188;
    uint256 constant IC23y = 12069973543474579174453374114184931563879493217159598885155878211675395470899;

    uint256 constant IC24x = 6678725852191442846507683265858781511233745665850250825731290811605250554416;
    uint256 constant IC24y = 18993526543812670463949277191810095089071608766533275688488346894774872035168;

    uint256 constant IC25x = 11351554936820177084495088953729943224985342763075701975160131857529099401237;
    uint256 constant IC25y = 11130948806654697978614138619528848325785238680312119467270314628226013808615;

    uint256 constant IC26x = 15191241141966413290995955518737515477361465523536622340486639063583303755553;
    uint256 constant IC26y = 4128104793955924385004695534897758111844431094080347694506976801400815317053;

    uint256 constant IC27x = 7453445003058438587393565102700338658759779362638832893286133625843075558232;
    uint256 constant IC27y = 9201973952925286749355758360137930261509288672503790067623440279981069766642;

    uint256 constant IC28x = 3958560201104244166782661481485467432453128096641490129040068721327757164829;
    uint256 constant IC28y = 4785186822026851605269411456190135012062818697212091464584042136311569573490;

    uint256 constant IC29x = 14135173956410254076250706352978888714890703460625920456644337487290994849043;
    uint256 constant IC29y = 8184808971199758388705478847973126943398395924595018517123147001951790399508;

    uint256 constant IC30x = 4855686201352628471964989610540919964999374306652600197397639894286251050681;
    uint256 constant IC30y = 14338443384741382618134421649088161885790749875697935979155957047867206690843;

    uint256 constant IC31x = 14118521211003466183568266816403517250188305893803964170370523975329161961290;
    uint256 constant IC31y = 4024772268461070526767989674870518016294312027731853603142980413243511225548;

    uint256 constant IC32x = 2016688635959633970501527061101314983755493682308750772031895890616399925288;
    uint256 constant IC32y = 20965070499934792849036219919486794068329601539692264507432120905853962096168;

    uint256 constant IC33x = 8141031691125019231889597428553027466400532115223613381814484079622656890513;
    uint256 constant IC33y = 18524551581133391869220142453029008237933865395482266845623780948492708787179;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC,
        uint[33] calldata _pubSignals
    ) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, q)) {
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

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
