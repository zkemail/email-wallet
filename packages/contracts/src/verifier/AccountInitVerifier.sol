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

contract AccountInitVerifier {
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
    uint256 constant deltax1 = 18530238972995019226136660590858875165795162000620050102873750403770349514570;
    uint256 constant deltax2 = 18295042382365107953468164121306019145677041381705919026747726340644142567632;
    uint256 constant deltay1 = 19396615805496321042035741138675265021327267075081042173580334824840675477023;
    uint256 constant deltay2 = 21802581312010443013931316230178986137412883786551926798951678238099113293405;

    
    uint256 constant IC0x = 16694267025811444053134683658397600453934903529986633412512873397650849266357;
    uint256 constant IC0y = 11093198574039342097781942424691751599392356763118998804346231567691093792891;
    
    uint256 constant IC1x = 3259732343904716870773107053117177704302133134930501895894256645328338879771;
    uint256 constant IC1y = 72601447389890258565939539589159802894322477554943549104142220351646521638;
    
    uint256 constant IC2x = 10854373259218122169899281866144525611582703203910786759396331474804138533184;
    uint256 constant IC2y = 3600532796833033180790950889993445084240527404769574287242290273984071036777;
    
    uint256 constant IC3x = 3897257880784846602309819239482560890274065109209569557257616752302913828625;
    uint256 constant IC3y = 16890529137263669939519606790922135085292608661520944923470039626219793734380;
    
    uint256 constant IC4x = 15362527103430792364581489648231710463851518328851873583727042195857132899168;
    uint256 constant IC4y = 4022580760756823708144528930927567597991403427132045846107468000587174714289;
    
    uint256 constant IC5x = 9380209515384258497900525404420295487459920205421376556588365617488699425679;
    uint256 constant IC5y = 12534333601073154654469300947194815570861723391383669166990564946893904776361;
    
    uint256 constant IC6x = 7323681346814069394192772132191297129371926486494283478742281509513819231649;
    uint256 constant IC6y = 4621058342194176206867764267184305112078175244609201968848583515171040082109;
    
    uint256 constant IC7x = 21181688224252076500114335690655138973813308600579515477120609564675941497150;
    uint256 constant IC7y = 4710059193805013079655328115008130763208867718672516785557275464303693510360;
    
    uint256 constant IC8x = 20356507679470449676490261945127664134379664841816049740672832738560274678607;
    uint256 constant IC8y = 15021686862922326255579338626393625556211642851739476920831614185767159944976;
    
    uint256 constant IC9x = 21086314118285409514344704290788631578368900453335767815169933061456892252739;
    uint256 constant IC9y = 11225626211991166791787171268312851676695720016432477258221463493324936467823;
    
    uint256 constant IC10x = 3683722151996594315138088958246726229774982149111405320657182349962260072220;
    uint256 constant IC10y = 17101560325532873206521966875612352375421606336890441533330782114291652739673;
    
    uint256 constant IC11x = 21530305486963676653707532238736805310675199158770788678424020065220272584394;
    uint256 constant IC11y = 17597535855692547231055613727867103966092439294143634009649842977785048076706;
    
    uint256 constant IC12x = 18853682564966449725425620886639104669634568440661138003114858294489375270295;
    uint256 constant IC12y = 20322438234616296468430934855245603184709214510018660106362772624828484063620;
    
    uint256 constant IC13x = 19277552714566417115239471788700031350985854471128256500988816561837373927028;
    uint256 constant IC13y = 8745730161987693490712517408865327009318099788822122467701755272306170778201;
    
    uint256 constant IC14x = 2053262972371179038842430001261209942098796926634370943056108257543237983542;
    uint256 constant IC14y = 8422533766715539547580270497004807961272972872428349610264074238521822670402;
    
    uint256 constant IC15x = 16702117976600869414486269748587206710831208868540311864360369706856489421135;
    uint256 constant IC15y = 12190735661316296793166797154904080664427670246133906158914368994616498627408;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[15] calldata _pubSignals) public view returns (bool) {
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
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
