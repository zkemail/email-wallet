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
    uint256 constant deltax1 = 21879771210433836765138988338404543990008078322878392397648779324111894235566;
    uint256 constant deltax2 = 9516750279424098971989614239905216904458131012290557215009783153893438137635;
    uint256 constant deltay1 = 16659924221331563317043408808877537517172532393753998061445020121984365042348;
    uint256 constant deltay2 = 6091420914425316074797742666493391385097368484277129363630590550756953348267;

    
    uint256 constant IC0x = 20697435302144478353780222657124466784737608101972071014311736133306888313056;
    uint256 constant IC0y = 15512257156330895265620318794911286578523145108894689778920618423296300507421;
    
    uint256 constant IC1x = 13380176851188552816140459922148664068453865178601262147267592067332842711268;
    uint256 constant IC1y = 4149674860679821042988943782634396343086423567216419311153888110980829170584;
    
    uint256 constant IC2x = 7564315138427028906509000744552417774046529868640992429668093999005526297421;
    uint256 constant IC2y = 18109033678958916971321391167851414331359740511825195802901592261738357437277;
    
    uint256 constant IC3x = 6330699836684974308161816627271087014600014461904621905321734555222026079968;
    uint256 constant IC3y = 20119846283359824948226000751868819767067141030829033619059214545629348579041;
    
    uint256 constant IC4x = 19064424297949479295619003056023656488738919578514628814442962662129824613925;
    uint256 constant IC4y = 6101083816936369776931291952416230558172854755418759180332562281918917794820;
    
    uint256 constant IC5x = 15184060061420779567051283348969506534398011159863908053348238289325304892616;
    uint256 constant IC5y = 5485178157467693813737050137489559037898217768534119290987687184354347451255;
    
    uint256 constant IC6x = 21506330720672802034238082925954187443893721399452394536136019354601949485047;
    uint256 constant IC6y = 20520357049465730013452579724523788089193242467901096477941358073642524716722;
    
    uint256 constant IC7x = 8665210386111912338395149107897521929651493926502245269684122734193546543145;
    uint256 constant IC7y = 15493161083645565443610694136513923142357884812052130122298421499332039655454;
    
    uint256 constant IC8x = 10685628533206672440850246980428275549050270220341592603782358917971866458623;
    uint256 constant IC8y = 18720256017887528060400826709935383365436394396917714799264861885825134847141;
    
    uint256 constant IC9x = 14322920753527556210095210844276076039744479058242427003553545988313676299080;
    uint256 constant IC9y = 2165708959492860320426906497653878139413295160550443236653035427683176348898;
    
    uint256 constant IC10x = 17853480405173133222855500964601478685312124564349751132912989699500995460392;
    uint256 constant IC10y = 526077000261754535685003355716194839618473480025533072220522371256130085776;
    
    uint256 constant IC11x = 14508561033101136724350174734198422118489402603775501658917688593944210286746;
    uint256 constant IC11y = 11050468833158089822111881462839244731630597033946193346663345030758291259254;
    
    uint256 constant IC12x = 17614967085923109908698175753791156008935786832509200164435146084595451753255;
    uint256 constant IC12y = 6283212701797291101350772237945279983187925457730282371321600683007002885411;
    
    uint256 constant IC13x = 7507372660002652766470822752342031481667442226017105043580468890242869263116;
    uint256 constant IC13y = 9678293632261753332282398997302114472234368419615861324712122949958331396309;
    
    uint256 constant IC14x = 9651491944335374492633797144941704791402227567049340096259378110301371630740;
    uint256 constant IC14y = 6937893161556368280545365159622876546187782850879898847074404289224250519626;
    
    uint256 constant IC15x = 4656190584660608268051313503152644000124303914695632121319696055334844389451;
    uint256 constant IC15y = 17352404848640958484768875505657908144636002780555866994066193155457599693099;
    
    uint256 constant IC16x = 14109367457165280512269805936356792259268073114352997894954029222217115708985;
    uint256 constant IC16y = 6116434028501721547438792942276961990165061388868489074007723613832222279807;
    
 
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
