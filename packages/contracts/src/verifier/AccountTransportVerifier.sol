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
    uint256 constant deltax1 = 667324179029003648170566394556821425906535947562566560412575934413215906427;
    uint256 constant deltax2 = 8942139639584024876595897959321001560961350948711779128837929744323693917637;
    uint256 constant deltay1 = 8807349864447072058665982405693862760648398808361128404375279503391516695576;
    uint256 constant deltay2 = 14675827071629138283426860928974249391898898116511001856003349332108308720785;

    
    uint256 constant IC0x = 4512569620770144159835924402689580650637513512532256610607224320353315152541;
    uint256 constant IC0y = 18903840103499333649881961491734535001641540958381983556752437402281439132645;
    
    uint256 constant IC1x = 5425062430153146173126112401756762584469013530909450056863582856761948166952;
    uint256 constant IC1y = 7494826306620990547772544985557979521704552409686757590115594478405834913929;
    
    uint256 constant IC2x = 18102780203671404921713120018835804341469153191916896192002929175106693135894;
    uint256 constant IC2y = 8854265293359151021475366990362640340972685827875584697405761109504839068947;
    
    uint256 constant IC3x = 8402457029951992842450328552314051095494375334112920853801228389039703492837;
    uint256 constant IC3y = 11960447621756227977350080270436492681541492620528828623937212762739748411466;
    
    uint256 constant IC4x = 9599480833064733051778128577533434711707765268017998714764702086205097040663;
    uint256 constant IC4y = 968062660061480704350697735128732787407621780255365426875734927376312643139;
    
    uint256 constant IC5x = 14858476393833185345650391349041362089287762378133082503918161742025446476641;
    uint256 constant IC5y = 5467673150616267681334324694335011769831102133006506781253070974685743259744;
    
    uint256 constant IC6x = 1636648317050006992878496964750266585964198625784139448951104282340991274522;
    uint256 constant IC6y = 2204303436982935341867241527639315411827745410145517008714001554717735646254;
    
    uint256 constant IC7x = 11868795222364158409745335584948647065161989937066890395274417818575713521069;
    uint256 constant IC7y = 17616386489502317509439166504548933665495257740705360661900961435361453000521;
    
    uint256 constant IC8x = 13540405037082504160982734817969115640732439210559261277645504619726120945796;
    uint256 constant IC8y = 19324308935178511169532160735038330499751581285858911890240927392210319480915;
    
    uint256 constant IC9x = 4006324096840062211293251422646221610473233435733383653619401600423862001509;
    uint256 constant IC9y = 2445479274649101769053683966168015051691447676439653888929162125226313554454;
    
    uint256 constant IC10x = 9833518265890331446835004338479950049780041668082953657727839321939395101105;
    uint256 constant IC10y = 13203441581641969700157199798943048599743367831669402466040661718826175800834;
    
    uint256 constant IC11x = 20996492454607854600665041178036594356696055757222254683488279986982696411396;
    uint256 constant IC11y = 15576196445562792177630028320969289724059001767685744771349084542155160403910;
    
    uint256 constant IC12x = 20568303134839673987533348555355292777257271959783828953536385191878582748535;
    uint256 constant IC12y = 10249004262462411765105814852745285913036599108846660870107311576159618457920;
    
    uint256 constant IC13x = 8055016069562449308238616774127387309484796957337220388844435747347816497250;
    uint256 constant IC13y = 8580105105945218440470198472485576575539650910017013413170975026542896794195;
    
    uint256 constant IC14x = 14373509517802095047796247832084397530814359191315143619797838902316522762143;
    uint256 constant IC14y = 2937503312406532431460048499548321498327682843634085145374828384063149055304;
    
    uint256 constant IC15x = 8801841118401786453855967139543885072668765730771928268091929032055197720865;
    uint256 constant IC15y = 510407356746429212148416766673912483631750443155933456232042817420456715000;
    
    uint256 constant IC16x = 7641624215014731232695913284706122984132627421867861338932770325906691385759;
    uint256 constant IC16y = 14345472004517844356153791998602709017282010922874690139976178437831349461281;
    
 
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
