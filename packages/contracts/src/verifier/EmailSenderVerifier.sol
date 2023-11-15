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
    uint256 constant deltax1 = 12786318381579463105938423975749437134643705027010297036699990496713821328031;
    uint256 constant deltax2 = 17302443221349671331694452107005609047515151695376759053749834262920969777100;
    uint256 constant deltay1 = 11508309516082526391033950689449559085521895345109988651027841797320260835250;
    uint256 constant deltay2 = 6898960063769985852170056583971763055326798494460262453090586654148678755286;

    
    uint256 constant IC0x = 9403939598261716924280279145444636577841588248394047635853056591499743123944;
    uint256 constant IC0y = 373367501944062315012229416723146138201142282083460199313876586227226939340;
    
    uint256 constant IC1x = 397078731139733248534550499545226210407043930195971139455680213686020548098;
    uint256 constant IC1y = 10785003777514996079226197355167820998262208085057867249117163004908404031740;
    
    uint256 constant IC2x = 428816242060318325126053712509610814268225822231322860293178816930522388837;
    uint256 constant IC2y = 366795745617245211365926501726188111003374151817054128373113582635986414233;
    
    uint256 constant IC3x = 20704499260755094197284959400998063555059965861620637289991149226396879848688;
    uint256 constant IC3y = 17526613354949780383089265175048648866463845796489846223326347994145743113523;
    
    uint256 constant IC4x = 13110590429346398414807288694544800445326793281935762473899244758990204276897;
    uint256 constant IC4y = 5397132242737774624652434115629942572364414880665567381668760078412648200815;
    
    uint256 constant IC5x = 17940243045025560179852022653920668342504730920360388781997296042669234928675;
    uint256 constant IC5y = 13124450934593089163577840977022809615792635321381614983618005485736932155829;
    
    uint256 constant IC6x = 1013802872817887558110897336544551692738033489382851258490684344783870280117;
    uint256 constant IC6y = 2358776531603152510491929132112282486043804761963544045677810572375787794989;
    
    uint256 constant IC7x = 4615838355591039824772105100205156056320725913486838257842792511043704762779;
    uint256 constant IC7y = 12261416047493013620181311705924790673619591615152872202549222937058527259230;
    
    uint256 constant IC8x = 11430126249839935594503500030144908663495154310678620888878078597340871678716;
    uint256 constant IC8y = 9479844111604973077230597245435107743651363130155135256193991469884075196391;
    
    uint256 constant IC9x = 19167318033452302279449757497911309579385883936727017507737482591817600543626;
    uint256 constant IC9y = 15376112179978699779834807151967448981563844079929010162652696437827135617451;
    
    uint256 constant IC10x = 15601254519259398663132990614386685107121117115011469304640859868379433081379;
    uint256 constant IC10y = 9911391786111790435276734342573582026358885980797215612947281993739208035928;
    
    uint256 constant IC11x = 18236330385201719660970334558974942992596685297852603424456051209224364923167;
    uint256 constant IC11y = 541907352275340999155549491032930070622474006226059581268848292905893058072;
    
    uint256 constant IC12x = 16340188854257232846672936081765938447280055681971990912435511909017118269858;
    uint256 constant IC12y = 14410225078137879222923701088233178167833283044484376243879795438872069944713;
    
    uint256 constant IC13x = 17114807060246417079689156940818608032584585333231311864219405619663171534011;
    uint256 constant IC13y = 18193165049734222959083784230797423326559877195561409512940643243027270526376;
    
    uint256 constant IC14x = 12716423269993471401198420003438471213267196159443828531423760940022466440931;
    uint256 constant IC14y = 9368029024836790156707039295414193266787634842342005782060027867224087585327;
    
    uint256 constant IC15x = 20372579536056752111327138677397409860868466768952981655517973827127970357568;
    uint256 constant IC15y = 12736281596884030771599262976768034399433409456726083303839166503941737284363;
    
    uint256 constant IC16x = 3605734274595005911092014464321637951766386206146315940851170928409996311874;
    uint256 constant IC16y = 20556673951903878497309969456564676516700017773608101078081178124578720586399;
    
    uint256 constant IC17x = 11852427753676934460883629558882898828535218312674726099280988846201534707663;
    uint256 constant IC17y = 17808647141312479262225789097579979892124491607468588466305886924936043510486;
    
    uint256 constant IC18x = 19359096952173409153017313102394180421345096434915851910427930465064688537612;
    uint256 constant IC18y = 9360859378302723458454759191100505257095386400910423555710631011971534060881;
    
    uint256 constant IC19x = 7180742168837350836802470446413397445732943091898009574006348837880502219867;
    uint256 constant IC19y = 2473490758365130964927658668646131298620715724369165835249228189164619048999;
    
    uint256 constant IC20x = 11539613684077354846454031489183060917978564424172397320017137356164095709959;
    uint256 constant IC20y = 2839449227294161837692545761804658150492778250175326842955241704356373174667;
    
    uint256 constant IC21x = 5206772780484807344150037479097128934696130589778199384705891955014618071218;
    uint256 constant IC21y = 1091176116177343976865027482851213009469197791621346141607583694277617778462;
    
    uint256 constant IC22x = 19998321292280342470577996436083191874936500778811205102838653881062364224793;
    uint256 constant IC22y = 8478518284815494434901253703461619703923008302389243334399599460937351674849;
    
    uint256 constant IC23x = 7715017723013595728451096100083109362968021760197471866519691512328855521407;
    uint256 constant IC23y = 11423439213429654214125344031630729869380648910923369696268303446892171524752;
    
    uint256 constant IC24x = 19706418075398854555841733881258301638853743029737688125112291132033406452803;
    uint256 constant IC24y = 10007830037217729390575586555797261662219650256199131035516570787320423975156;
    
    uint256 constant IC25x = 171860064829261027599160644461017537028148903252719465590602978138879076606;
    uint256 constant IC25y = 21609224961063042403326113961432895254727561184198389001281394840678861867222;
    
    uint256 constant IC26x = 7284547673844651976206119977064812756062801457320127655490384761685707321581;
    uint256 constant IC26y = 361471381855438584032066480061699712544821631218099706126221625253574323679;
    
    uint256 constant IC27x = 2392572711451769566262600310208372147776031061482590515160977412312833184568;
    uint256 constant IC27y = 16148194545044715811076553344938746016059785651211257087702813691186098628868;
    
    uint256 constant IC28x = 12498115311316671482936631575705738890115144294477876861339048659123811831967;
    uint256 constant IC28y = 15544067457843653635759233767064866591650320842452220932420267743536551788696;
    
    uint256 constant IC29x = 2261291525942770196342226952964533283138642947136846855131068746013271264453;
    uint256 constant IC29y = 9215191185802284726880776758866699019940136364106043055297214679111858845966;
    
    uint256 constant IC30x = 10521008861135201728739547853501050229024960017803391967209714476158931627968;
    uint256 constant IC30y = 19701639274009038556443243925301757302246914150473631593195853385705998008881;
    
    uint256 constant IC31x = 16050821230919033312377420970766699524999668441495877997480488092051508378578;
    uint256 constant IC31y = 5420358691592742252504747514758033971323433011223087945229622340903379807727;
    
    uint256 constant IC32x = 17730297954860877895842611792171043646264888120505659624912028096015369181938;
    uint256 constant IC32y = 4703196290737522500778098967747549482532720282863078103690392981875407783196;
    
    uint256 constant IC33x = 9296552111467290821862868828372692550604701677564193669017235745622158179666;
    uint256 constant IC33y = 13010296259180220803277181694766269726519061954982896869385123923773021285455;
    
 
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
