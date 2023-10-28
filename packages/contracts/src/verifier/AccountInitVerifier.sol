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
    uint256 constant deltax1 = 14452753850470211690956472250930593509111137006488757022806843717301963659379;
    uint256 constant deltax2 = 4716341000363531545534703465520079046261526317074041264741454108301180329467;
    uint256 constant deltay1 = 7972831988715004314773197648597026027449105851209902254471000253197712467744;
    uint256 constant deltay2 = 18825373937438342363544174301573496526258742722296880801493303177381165333053;

    
    uint256 constant IC0x = 14707009234681939697015515328312611140300097352151132879074065897812345510666;
    uint256 constant IC0y = 3898135439296651331588476163439242279179860036207207112449709233604238246632;
    
    uint256 constant IC1x = 20137370945993935001101019258961075425689319280907056041635119871203228868031;
    uint256 constant IC1y = 1046609136759462581803590437183322405719357915637333829626865277374371968247;
    
    uint256 constant IC2x = 19392610968901245477219570166460519209745967530267069296744819880192076444649;
    uint256 constant IC2y = 15997704576680269428527632744073058000831294262978740618471050688724553024171;
    
    uint256 constant IC3x = 916942244716485640089942354719862103007251648016746741386071367348671861471;
    uint256 constant IC3y = 14244404990872610288197473402513597660350282485186043633335807255698860386781;
    
    uint256 constant IC4x = 19039214456805669515269507341185340445193973525192327844072107419779507654754;
    uint256 constant IC4y = 10267570362036791459813070228599838002897180117089432685284542525980886320351;
    
    uint256 constant IC5x = 18471461350694970546274331079997053657619332679990885861095040528606249243652;
    uint256 constant IC5y = 238072647796111545813081047299961494175829179936550880371384059114829702086;
    
    uint256 constant IC6x = 1083948915130763452703656607092574888680573382034443202339970654222927983742;
    uint256 constant IC6y = 12326583033458122728595694592483848907089729102805432372278267868876333111144;
    
    uint256 constant IC7x = 1673861979628053970159336210118401192429335963767105307281469328317685352792;
    uint256 constant IC7y = 11596416250376738314259510807612889893589181398005749611025064516121722459911;
    
    uint256 constant IC8x = 4782931301394205791720181173679117203011439041454661328694501718806967571332;
    uint256 constant IC8y = 5171585563956145195938965182558144475463361735225104118702736230985237423361;
    
    uint256 constant IC9x = 20759676246379864915118502942558459771432855002943758833790332726278044862439;
    uint256 constant IC9y = 1350334760191221416038190524088233047955913982230849370313882043289618347648;
    
    uint256 constant IC10x = 12940613891955858091290942289825228681413021069049204292108996436481550692985;
    uint256 constant IC10y = 15393966844282497257051312666228818424749315465012406220231837260084255268909;
    
    uint256 constant IC11x = 20515501640991398053120267983448795178144727046625103091677767240595141545190;
    uint256 constant IC11y = 21315528799645954846952084762717427349778880990113368739663660856232279897388;
    
    uint256 constant IC12x = 19331123348045982047962334905496074840235816710835398126272720462187854844616;
    uint256 constant IC12y = 4616348180590777428035287970595341744709210867765337104269586379775472807212;
    
    uint256 constant IC13x = 4200403575918142129904567551879972159304646643851897387788628109562275252067;
    uint256 constant IC13y = 4456330294328220104370302396164264521771678598093594815007321421471942267683;
    
    uint256 constant IC14x = 2066147776218536238520864133914963987073113962156345158399549042118824757616;
    uint256 constant IC14y = 811610665915246069164627505260044779050709843076957073945863663814728189715;
    
    uint256 constant IC15x = 19589000511625754209112258143891532546120023858923591228080712657899195572217;
    uint256 constant IC15y = 12486158258872820039512786903925315477183746539850062166390991921304667075731;
    
 
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
