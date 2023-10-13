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
    uint256 constant deltax1 = 1174151490809310407813992218051612888082561203038036591585602786009102324744;
    uint256 constant deltax2 = 9894086689002669931928966320509956881106819103814734660024191927413372204828;
    uint256 constant deltay1 = 13118236169165873264522901495260127929550285912371669220370900019141170009292;
    uint256 constant deltay2 = 13160899714289725184525305465737109412568986469296409001767972398810308698992;

    
    uint256 constant IC0x = 20971710200842604111127636739803671638653882008689771824313250934147054224690;
    uint256 constant IC0y = 5746482894353910987326544960737402701204409973331769025813477374461901557594;
    
    uint256 constant IC1x = 8712150727337000249690717887348342805445955058329432053847470791665950129645;
    uint256 constant IC1y = 14259649401815032605049258925142758850837237328318828776216652285540607798300;
    
    uint256 constant IC2x = 13008133120017868456702086199998802118530787344299047958463997141245020276821;
    uint256 constant IC2y = 10770564560576866411519362243959743150203023329320557755860588465114320141746;
    
    uint256 constant IC3x = 18176934773564811463129407753772065855473421930735700956054093671731523147285;
    uint256 constant IC3y = 9243002477761707476399672277339201820884824632277735190060264242855896387616;
    
    uint256 constant IC4x = 18750915380181644820138085030616774949741379102240864902488878036065811128594;
    uint256 constant IC4y = 18801664223146682021864580439975944116777372972437586675194151060436739232444;
    
    uint256 constant IC5x = 5571129184543185673025983206920494476400372124970899539184983762605117143176;
    uint256 constant IC5y = 8515967531131413030042882750561754330865773575762690127593193094508362905694;
    
    uint256 constant IC6x = 13924196431575162085207908956334504067977324076435142118514260116642773638165;
    uint256 constant IC6y = 10786051946753297777957613460347870832098634758798936941474393709764463271927;
    
    uint256 constant IC7x = 12818470448046422614657344730818185075576935554918106154726981388055170935109;
    uint256 constant IC7y = 6187621591515151523909773472608243425652563576967471180633322538903471072261;
    
    uint256 constant IC8x = 2675207975419854240112895820600429004186575844602297780365323773406317669406;
    uint256 constant IC8y = 17493594857557524830350774262650156059354518650415906332959897075210251217128;
    
    uint256 constant IC9x = 12787332748472293259891988509135894268404557430618504799317239040574290631411;
    uint256 constant IC9y = 6286200792951170384615816803120773503295056982396343357544718319462246716999;
    
    uint256 constant IC10x = 994671724469814940247612634236531144117392280487402532593822515869630626791;
    uint256 constant IC10y = 1948028756218102405909202288348810797661951842026900987350305572079715554607;
    
    uint256 constant IC11x = 16291652372821767621018568922689804472860176900645731312669768316671027119943;
    uint256 constant IC11y = 21746778188856893308567016368599979495093790588181570103469165959332074111775;
    
    uint256 constant IC12x = 12940327871550226944526640474199530468627616766285767854243135270899226875281;
    uint256 constant IC12y = 18903578681313464025143688925785878718979663763292801160706166581172231717370;
    
    uint256 constant IC13x = 3656722259768972851200730567296489267260368782458561514518509792947887093895;
    uint256 constant IC13y = 7341179044780868099523435583560600173498904541778772290343098215899367021872;
    
    uint256 constant IC14x = 9314396370455428238039256703395557688236199966875677428312913517486090498926;
    uint256 constant IC14y = 4886664724911700583515904399833258496514545691184791659805333853998798589515;
    
    uint256 constant IC15x = 12634022753475602317960131885724520238836506040569049348991770079581188468184;
    uint256 constant IC15y = 12790518230771527836369423322449605217466966728666581856912623187219380804424;
    
 
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
