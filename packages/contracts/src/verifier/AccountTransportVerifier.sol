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

contract AccountTransportVerifier {
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
    uint256 constant deltax1 = 11169571552076517348110758269953318360755635867440844010909816980350956910525;
    uint256 constant deltax2 = 10308560610188361405410747180157378850350510449826159647149321634240762388611;
    uint256 constant deltay1 = 13162619938042878480790884435088314197527002431486153714660581538152524334420;
    uint256 constant deltay2 = 3334258330395016248100766786130389326347480182394020819023761146769584232633;

    
    uint256 constant IC0x = 10905465849847747666399218918117106387533039413114836763030514087003502647985;
    uint256 constant IC0y = 17783096667725195540122734967289849046116641063558243268778087773402140733980;
    
    uint256 constant IC1x = 9603196186921916344665884125654606320946007288735431859640617168577335054994;
    uint256 constant IC1y = 9362976248241644332164154214984237235235321214490141931228466007682934328720;
    
    uint256 constant IC2x = 12806717888288590947390171539225186201080381901745761562386507796479054214660;
    uint256 constant IC2y = 209177475916124028519143579239223555330832559056404680055425231552270591311;
    
    uint256 constant IC3x = 4888548050665717386023596116918646129350522485210357998718679127361647261915;
    uint256 constant IC3y = 19112002853592122791834126264440756349278389377445488279250821451353408605667;
    
    uint256 constant IC4x = 15620338666272659616797973799820182794264919073503404276920667387461240948639;
    uint256 constant IC4y = 4932268283281222159586213705046432497173371846614097790814563573090910847384;
    
    uint256 constant IC5x = 5440484121603370524491195416828142832144414332374756264286073841638142928856;
    uint256 constant IC5y = 9682926064023833447241934806401436302064780260320490747452927312636046617016;
    
    uint256 constant IC6x = 20560537672044881194057709380974914347300879061779458904436201550978220254182;
    uint256 constant IC6y = 7965224710285599088217478890148233916216159574469984198046283259718610081883;
    
    uint256 constant IC7x = 3169145211123886297625991989022339582378947564151273324613318368345149793595;
    uint256 constant IC7y = 20728076304517057122624451832460971345710769348048786615721928120341936153814;
    
    uint256 constant IC8x = 19352751669761105957278227758847533464653938559405066709118850241766800806132;
    uint256 constant IC8y = 5857892090700439155593502411430962936105638012439680133704866955810162930086;
    
    uint256 constant IC9x = 19513812433949188241929106949274043146982324989789331493553866260878353987422;
    uint256 constant IC9y = 20376803407869130806451911977778816627925772421530722391773187354466081050999;
    
    uint256 constant IC10x = 1773825637199364967668632586494913890961997190052842254849427940495296399490;
    uint256 constant IC10y = 8855371124921111589423088119232933461884104036365325571306657095189198672891;
    
    uint256 constant IC11x = 12115275306555019526875648841817863649629646298563364699691276961158135115096;
    uint256 constant IC11y = 9241640763436951192871191514729484725124058326591550242313370627874903007218;
    
    uint256 constant IC12x = 11499806259958813495855917961138395867914780394274478936992696873976972418062;
    uint256 constant IC12y = 17886493279633921097978442103817779896841970957397242958529022051802145788626;
    
    uint256 constant IC13x = 4674379054536621980391738546165420860315646199934459773682848393216256303074;
    uint256 constant IC13y = 6349686515728207853542214124540613508918372373264556469998233610465769118433;
    
    uint256 constant IC14x = 2391565183084277931272263475376013369932586361802212942951840581820364706455;
    uint256 constant IC14y = 17894479394673972656047994707206352625130747109561508443008205096089229863819;
    
    uint256 constant IC15x = 3645491810836198482257562126486669417493441738139949801757456065447448827909;
    uint256 constant IC15y = 1054825897161494908306505896031982629529192314269975399754643902850268309275;
    
    uint256 constant IC16x = 17444819289212934795010422220080411846303793762711030683230904682260339930714;
    uint256 constant IC16y = 14611040734026153357812799049932196906468706356977952010784122057864865913692;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[16] calldata _pubSignals) public view returns (bool) {
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
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
