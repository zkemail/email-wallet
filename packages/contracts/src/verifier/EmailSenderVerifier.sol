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
    uint256 constant deltax1 = 11346063425972635777759645314396872584038743618714975962855456791602232171092;
    uint256 constant deltax2 = 3504277632477348828172683338184915117218023340635675461232409992535012073091;
    uint256 constant deltay1 = 6293327860973313693918573248362454717002769113104152982644914025441842993970;
    uint256 constant deltay2 = 17222157408217362594468531123626359765486591687913177014402299971148939264833;

    
    uint256 constant IC0x = 20938196994125795611069829947400737798490507150334496857661824851544422611757;
    uint256 constant IC0y = 7782370039460383548499655773746205652810974120448014387057150307483014390817;
    
    uint256 constant IC1x = 8370767296827998805576667302683058379976413028338728235574790043010818377201;
    uint256 constant IC1y = 688960423701529572084748332966779167927823637218091010772723348413581459027;
    
    uint256 constant IC2x = 20665192572367521670604347504918002293766018956470919769457285806220316735173;
    uint256 constant IC2y = 13051189872134544576549058380696953987368769375984359896060341093425090739469;
    
    uint256 constant IC3x = 13541671509878533978071361716132087006043188319295154911524823929485148109594;
    uint256 constant IC3y = 10245138924207086597774957488087924867512106833236522960696217977103097506921;
    
    uint256 constant IC4x = 20478522330396011436882281944520899080750157538833871478445004086184587892093;
    uint256 constant IC4y = 20350543419749541248074792975444514455230590999399226737707417448840511640079;
    
    uint256 constant IC5x = 17422821812703775616385186449624764139761647539400589862557563888491836307060;
    uint256 constant IC5y = 2014524423975027022337476305438873639168190853652863158148358793777481488946;
    
    uint256 constant IC6x = 6363490290974086610048627088222401647117922096744558859431234429332130055006;
    uint256 constant IC6y = 13344159082697278108516428737138756701756849331823754944420315592824468677110;
    
    uint256 constant IC7x = 9501745256798055097735043606218391262001169796331497312233653451979366691582;
    uint256 constant IC7y = 335766647208180628222110208155227730389308823625764029124155854748605315163;
    
    uint256 constant IC8x = 17404634835236761375547038584443906383817408077098583499796014086764717121208;
    uint256 constant IC8y = 7159480136140969932465250859850449051092615897621937590250111873516976284097;
    
    uint256 constant IC9x = 9277498233195769095365882493641377072343726535359036990161806512548153519137;
    uint256 constant IC9y = 7819370781615961752983472554132195987634144624582196938714066768485055284596;
    
    uint256 constant IC10x = 12922445064955256685942686919759876880457003229077476224608695692495417889457;
    uint256 constant IC10y = 21113407210390374966832098431925602887842959622549217831607341810045159490068;
    
    uint256 constant IC11x = 13121140630434484840918981710001307948605924082641250706167496651558357226960;
    uint256 constant IC11y = 17781930507070344357934224941132354857195426161367471531796730113141986731183;
    
    uint256 constant IC12x = 421705721279367261167601480827056883147281850359447284150107972222881449272;
    uint256 constant IC12y = 11985166119012557219233561447413644358306806484704004477360187321288632803180;
    
    uint256 constant IC13x = 15206517140098998029353627366334716838684261977621842178718571122825053268971;
    uint256 constant IC13y = 9526235397262004194400610549541641378365997692022906523549557688300326907722;
    
    uint256 constant IC14x = 6862002084764230707766917016747853817645794497783882053937515501698298168281;
    uint256 constant IC14y = 14808372110065152298212461889145092860804411606602165494365341770136678274384;
    
    uint256 constant IC15x = 14502432033617752293832287362205587583718778312955536103637669759673341522900;
    uint256 constant IC15y = 2302620500086043473179147626615898733719486394280496278469437008184936949811;
    
    uint256 constant IC16x = 21268486036210504768269488551708164364133933108704844900514137717521326559662;
    uint256 constant IC16y = 12607444351308424143312048165741175828514581460037322334499769263640546255822;
    
    uint256 constant IC17x = 11916403994594301717403641143771059918475060146416130520314439601552009515175;
    uint256 constant IC17y = 4773265691826024212350670044521728974637774660315140632642374915534397816337;
    
    uint256 constant IC18x = 17233709403448221234737856593488412038894941592081571865476605080157243202771;
    uint256 constant IC18y = 7228881178906367847518585038745818265758308051040018743899475569452044644683;
    
    uint256 constant IC19x = 7015661491894764801464173519635629745061343721531868177924399201295531112189;
    uint256 constant IC19y = 3407591556296081769020450913332215731014221767243799474407876974705620702761;
    
    uint256 constant IC20x = 19762435131459224124332942287142795818500966132596357362182796668762443493724;
    uint256 constant IC20y = 17583275802630782361228542632472837956136755476671411433806302926116057731723;
    
    uint256 constant IC21x = 3371450231403377918708716031420103660247472517032517327571755781161128528645;
    uint256 constant IC21y = 16265754049268106168792467296019700809692821143818388806037910162176987351932;
    
    uint256 constant IC22x = 12721294476493210431840809029766878871913919979185650689144136799347775164199;
    uint256 constant IC22y = 449271405926555158540246671315765569919259267348910302160935220838280007746;
    
    uint256 constant IC23x = 10222723743888633005605650671144095505246895791363298314471649552778072645893;
    uint256 constant IC23y = 17651385917092180699396904596463848232772906656073448642150934904990995487713;
    
    uint256 constant IC24x = 4457373505896547774623254321939132378013059109658252661966236804992709508066;
    uint256 constant IC24y = 5164802051898790137990936557490775973165759407722442192007835896377504155784;
    
    uint256 constant IC25x = 20590792835850277386599004873046914869441591365194397515849563243795344374855;
    uint256 constant IC25y = 16453104800875903146107180758652827049710443896331310627633823785906565151634;
    
    uint256 constant IC26x = 8548855164677853277991347823797853426141639944322807585803709935056537046390;
    uint256 constant IC26y = 9976834047928324698405018659105436321883608273851800726077530371157395485091;
    
    uint256 constant IC27x = 8553855524712103162300727960861827000834284077143329287006409082859412323636;
    uint256 constant IC27y = 19023315401733894377904918248950962893542927361924585437794265724548673714611;
    
    uint256 constant IC28x = 9001029018342730039768094938569771949447557771139460416111406398462760393711;
    uint256 constant IC28y = 20858754043742485448135008196235484322431305369155737002043007163140443060846;
    
    uint256 constant IC29x = 17744437062750987990050034923699952326630955901265203340140749556018264199484;
    uint256 constant IC29y = 14990978704917047340258529570820005676270626606456744298284564998096811593267;
    
    uint256 constant IC30x = 15379334947285031559479274883829455880098972357501974191400870405156804503665;
    uint256 constant IC30y = 15497009640964355983255612937494065547371910265090259037695620804183455650948;
    
    uint256 constant IC31x = 1962052490842001357560648197875732035059504368423902382957103698910115533995;
    uint256 constant IC31y = 1706960224485509701088724935958303299210284874184244558111121439866367111567;
    
    uint256 constant IC32x = 7394560745544786780372421204735290967882304264999065259840876083923025563444;
    uint256 constant IC32y = 18613852553019423728133270921731844216517648639608985003469389780295791635092;
    
    uint256 constant IC33x = 7638102677975422495855839145757141510894864084752152695691605285589996592198;
    uint256 constant IC33y = 1064439281927768567443387709078196356557439626605523963403134329762546761559;
    
 
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
