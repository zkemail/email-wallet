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
    uint256 constant deltax1 = 3287848536573049637855573036982615864412705458202501662138146733726231440123;
    uint256 constant deltax2 = 21224472951017980818539415856441415108251249138537385756928333816446295929887;
    uint256 constant deltay1 = 8283500949653095541349576000838132734510381413836045721677046931786128957449;
    uint256 constant deltay2 = 19301827677329534975044593152196935353852211329569891468462493185376257025471;

    
    uint256 constant IC0x = 8273170599895483385064776761654624266498277466681957406441106747409138961559;
    uint256 constant IC0y = 15822764881959698276353278768828197365358404175723197503087122254643360201056;
    
    uint256 constant IC1x = 15717735736293607870193221349454847783758041022399723058879552310908842505524;
    uint256 constant IC1y = 262284394637234169022447846587079274630178661935414703948317749141910143208;
    
    uint256 constant IC2x = 1491480214198965124895509426386517867519614611502255007166398596205196205171;
    uint256 constant IC2y = 5742559454378064129938630366161829642316455176788826493167565650512323386955;
    
    uint256 constant IC3x = 2049170320541175573590924736292571093894227954897768853819921813933587695725;
    uint256 constant IC3y = 13602407328660247329004330655842911741694507411713967595493470110039866735821;
    
    uint256 constant IC4x = 14377436339154777473855850225926326558688877930910156234526698257243441511559;
    uint256 constant IC4y = 8640809499327326318372588064413249210130662496321590378002727947993797963408;
    
    uint256 constant IC5x = 8683090994296866088480856264467103162273324230054766119993261205059787870033;
    uint256 constant IC5y = 7689755387663045482806485686167179302381193706506746620288812412803442264059;
    
    uint256 constant IC6x = 20251422368543753138023873229717155294390845399248983034609781831388846333973;
    uint256 constant IC6y = 17858587734631830051380938275004091267635060814960918693359968668587218819758;
    
    uint256 constant IC7x = 1359286068292703286262595370327188009799035735342451011948877019950423015144;
    uint256 constant IC7y = 19587939253846447625386502871666462344184095605259331025505298020048081196992;
    
    uint256 constant IC8x = 6278871881445103891496157614733295365763632565842148296559234191069265920416;
    uint256 constant IC8y = 9596878447098369630442467793435442481500957384983901921300730922405787443505;
    
    uint256 constant IC9x = 16961701167357877707354910294783367841534896982833773362881687538396182150594;
    uint256 constant IC9y = 11482925767874379661106057101246338792495415350626517243838596273909269718111;
    
    uint256 constant IC10x = 2732221597229126953354225599734250742583852631535335779930035340043876008153;
    uint256 constant IC10y = 21423923143830439794740642982913093197983281981026286813809007185355691494447;
    
    uint256 constant IC11x = 21803907087010597457663054006255397837300591559536538443166792655549097153531;
    uint256 constant IC11y = 14775823163932082062537503376217488383396921880834180249849799486194757201340;
    
    uint256 constant IC12x = 3656405408726530153479199034361984337300616092034257634735678479267013425384;
    uint256 constant IC12y = 9511577526039873520651652419665911871764538871834466778176466638386116704474;
    
    uint256 constant IC13x = 2567893304330950012928547253219664111735050863740260811243598521047552889578;
    uint256 constant IC13y = 19017125861055681914213645636622019866681106255198431290895438015172738595831;
    
    uint256 constant IC14x = 2981456257427565140801249606962829766851741186106034582561365931836181713944;
    uint256 constant IC14y = 2728366357572061287097167604754876413525550845219469074625518655562618748881;
    
    uint256 constant IC15x = 21603566645551070678870992724723386085885778382987732722490874852306867724912;
    uint256 constant IC15y = 736047898771712118196329081121267442859879678818639959617930957661745963940;
    
 
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
