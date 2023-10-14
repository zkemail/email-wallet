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
    uint256 constant r    = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q   = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax  = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay  = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1  = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2  = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1  = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2  = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 9470593352236964338663312347791063500018355593799479172638949572110019832000;
    uint256 constant deltax2 = 10982466734716730638274309119404300874348546030661880372601678696908031970787;
    uint256 constant deltay1 = 18606576171874355532403574803056356139858474912373765004786274022481822601494;
    uint256 constant deltay2 = 14901927482976203827165256490284567164324606019048592926391741380750094940369;

    
    uint256 constant IC0x = 21590631109573066691930154938515499031830481534851543025916893152878697666607;
    uint256 constant IC0y = 4408328652321675346745819645586568528164036792056938407409647492439426924075;
    
    uint256 constant IC1x = 4769010776681681732003306783988575834689105334057108604444720254558799144632;
    uint256 constant IC1y = 4175274193617161167597012259014157032547778976464430914600930643524723352998;
    
    uint256 constant IC2x = 14115769793261146695885942945399276911322687820164254143072053786984801004203;
    uint256 constant IC2y = 13825986288835946140707635204145410258062640639189384942019691202704630348416;
    
    uint256 constant IC3x = 9565608017437677801773039794197901301301924419518730196847139696063956150548;
    uint256 constant IC3y = 11953011714328832480522137026812085659999653671845061039380899346287872371797;
    
    uint256 constant IC4x = 11430739111159461804138658868434294131343132119671470547579947140288882216125;
    uint256 constant IC4y = 15815887845900770566652810500338392657598244817055787245727208503484254467273;
    
    uint256 constant IC5x = 12881387817708148305340905819415796670424594735766067960040360040796466676645;
    uint256 constant IC5y = 99026036140010000564995843739502074328312531282257012607463140377152214717;
    
    uint256 constant IC6x = 3244258265396728714035380472954771182558367139252389163641421993237011870305;
    uint256 constant IC6y = 7073449467218156929179490542986138301473995760396693465422594725227489221675;
    
    uint256 constant IC7x = 19268676492388811591670357820711942267579797231105251019088388999935953552249;
    uint256 constant IC7y = 12460780946551558780074391892710599721355054062887770821518973948609148953674;
    
    uint256 constant IC8x = 3502869706299881175751867933637491145752672492675466851957404194896003262368;
    uint256 constant IC8y = 863856767812734833221151007444207199501980023879625479493837631451453240075;
    
    uint256 constant IC9x = 6291927632933876997015570109680811807489293682212951577180615184246340832461;
    uint256 constant IC9y = 13623444756037906881235066928418812940070806284391171829741110220519126222244;
    
    uint256 constant IC10x = 8358566172496744501038144062543685643164963500863728756649947098475790806429;
    uint256 constant IC10y = 4315755182403891861771580454389661760749371868918981789171921630516705659788;
    
    uint256 constant IC11x = 11361332217516772399897861394197522986981004879833074842410959331444305183295;
    uint256 constant IC11y = 15231400501595503152967749429824222754740094648948802896918112840734975658479;
    
    uint256 constant IC12x = 4554005449436697025454524253228801911500965428945780694406600277527803021852;
    uint256 constant IC12y = 10480671473896188285456191779824184282438006820833625028715159440473261646253;
    
    uint256 constant IC13x = 21581296873156269236431165270388749704344637674188119591693941943721930641385;
    uint256 constant IC13y = 5300292727815180166519318042881468407512650362023413202350953862624177156200;
    
    uint256 constant IC14x = 20526458742813903799873869665695763723913824152755819055963019881165557697078;
    uint256 constant IC14y = 16707077833182229397417985530853902806001436160986078618962295233997467369568;
    
    uint256 constant IC15x = 12292947728115185474141486844277369394406125690462079591412139648237497785565;
    uint256 constant IC15y = 21287480752231899589652157075253519035754503403221125912532530568707388575413;
    
    uint256 constant IC16x = 19345701996395289066071243346607852697875172934697143078366371374775474551451;
    uint256 constant IC16y = 20429229945952440906320413454752704794633211413619076934762174008736573116914;
    
    uint256 constant IC17x = 4930756897513799930911764757413955846411636179849766532899023697988837402975;
    uint256 constant IC17y = 4772044287911047660798925476229311509225986912568850464932137391414252109030;
    
    uint256 constant IC18x = 18385673803555717237144311749698699808080014141548908155401479706345837815728;
    uint256 constant IC18y = 18345295764899341791225597347175565539451827053748618556626828793850063055027;
    
    uint256 constant IC19x = 17072987339223743880000040206178512915136561385574942878307662514510310668404;
    uint256 constant IC19y = 1352618536732504247633642896687036225940138656848162978727931639072851683906;
    
    uint256 constant IC20x = 3490719419493247118097950741369168227436877323781505939238783469325396100085;
    uint256 constant IC20y = 2690045187417131871527443907864494708279580239661646109482252765569052206610;
    
    uint256 constant IC21x = 9778802450077399283321158184705911045326570526418831651303893635174989739661;
    uint256 constant IC21y = 8747593086031452939172272239911281866000254837504380089589935781930316199461;
    
    uint256 constant IC22x = 16962780337295466165687242663928599072786316822747935538404746474958229929881;
    uint256 constant IC22y = 2753366102044722346955306547338939051312113977655313495609658616984646947328;
    
    uint256 constant IC23x = 9246516382359567913942871288487859909506882677076640962925575016853003157531;
    uint256 constant IC23y = 3818045931489833923827067894278534284382067098702159459071065929247558853239;
    
    uint256 constant IC24x = 11216815257559249852446970390776811554461085717954227849879470205657610548013;
    uint256 constant IC24y = 12316630873202746098636648304639651245783904818792413353033860789769603570690;
    
    uint256 constant IC25x = 6801075468412787852693573515754314635904726146029942293520387409426926920902;
    uint256 constant IC25y = 9708268738657937617182439482334062719027320170715344820626068168591362201409;
    
    uint256 constant IC26x = 1183595691986496929931775846285950913070884364308979064709848478562111867681;
    uint256 constant IC26y = 3192175730870139078028909164786815056225292497480050829785302388973438268426;
    
    uint256 constant IC27x = 9001054657900968164123225300949476487494220781605219090188929816584992916378;
    uint256 constant IC27y = 8158142534995620118894100568523369108702086407896931873187366275131401678815;
    
    uint256 constant IC28x = 2127302585167211511299947814831910858391069965099831428023216176041203611018;
    uint256 constant IC28y = 14770008349608138973246094735984553547794453189445063432539611882691247892362;
    
    uint256 constant IC29x = 16902937041119136280626606221576987048100631782108762182409427565126829377929;
    uint256 constant IC29y = 9453761211095474764645078375204601353677259307056877140494252235247446652878;
    
    uint256 constant IC30x = 3662289017276798594056937875471271227263335444637212211310035451030739334866;
    uint256 constant IC30y = 16054599453624019686146345550498921784600793807768776134632422688762164893758;
    
    uint256 constant IC31x = 3565622168977030490784484858921616624290097117285847908860441928540677485263;
    uint256 constant IC31y = 6214895361538194077263772925640229866185008694474608735347222902587529643533;
    
    uint256 constant IC32x = 16661393861904903997224156056370671487161495878818360058069823680357725230729;
    uint256 constant IC32y = 19759466185437834705205008802127482805824501491050325019110693226918109981204;
    
    uint256 constant IC33x = 3486647807379409059949227946245509204173803956081270157581670360093286634230;
    uint256 constant IC33y = 2449792077303311031655802938374471246369723900705944542590642280377853852931;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[33] calldata _pubSignals) public view returns (bool) {
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
