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
    uint256 constant deltax1 = 12536259159160269990205137925682519585112823836894589588964127384745270855934;
    uint256 constant deltax2 = 7456371097529730174661450970738439773095620074190379056959808513202238012126;
    uint256 constant deltay1 = 3144317056837150630069086150182768328327905460178113520673338910068143984648;
    uint256 constant deltay2 = 20782887938752484609044137225966157686377586180568483891185122656560536659545;

    
    uint256 constant IC0x = 18743871196761584807479968380184479599934854281340859795449339149426463213659;
    uint256 constant IC0y = 17013435743956107779718674919444553904980823044101039517721312048577216273585;
    
    uint256 constant IC1x = 17096949859815036908954856162439379518117299459765395066866937095814674419559;
    uint256 constant IC1y = 18510246199848678520015682528480043742178711220308983048548937244216422493180;
    
    uint256 constant IC2x = 1895433984303013907183290380729292539593490339047189369329572142123284184463;
    uint256 constant IC2y = 18461736201981552484310774395857058365761177963791656917603965093766996340130;
    
    uint256 constant IC3x = 5311707608951881659139886737565760112449822507563012178821163838776886960824;
    uint256 constant IC3y = 4111259797949993873514486445489851110575339768857369151827863649001836497083;
    
    uint256 constant IC4x = 514389189381085017643573899515569024586864155001922056412457884937802524820;
    uint256 constant IC4y = 4160772785837761835614107097319707291755996044913142711908700062890691788492;
    
    uint256 constant IC5x = 9388920347294786432475355587733949057991416258931987467222858611365856823257;
    uint256 constant IC5y = 15479990231191176342888600546792400112800841532810122538367082128160575987826;
    
    uint256 constant IC6x = 17506694907903104458281233278696789152645897387862180267184378174112786645035;
    uint256 constant IC6y = 4293180359262538499877610841594117547300593164052819794672574257890163173154;
    
    uint256 constant IC7x = 2352431360158349612889378318296887046084551196707496003142106315475605345201;
    uint256 constant IC7y = 16898732923893909577151071041923602713156613403324330762257901384895231505326;
    
    uint256 constant IC8x = 3120317684862883295359296306616106051868422722061724922011317922281841152379;
    uint256 constant IC8y = 15102114837932992168012033919462313907863880242152092997420173517306343573785;
    
    uint256 constant IC9x = 14018255528616805747418445193572337169380511165862256554903332646929201196918;
    uint256 constant IC9y = 8744765717551935538145957744306441645655864242846693730340992908507192291706;
    
    uint256 constant IC10x = 18393645057548310929216041341964205244460109511257070655981029485602077699831;
    uint256 constant IC10y = 1213975383805121105671439846088577562160265647196980938704929134070409565551;
    
    uint256 constant IC11x = 19310138810334643315826184191536375203312102815351190347727688574311877579469;
    uint256 constant IC11y = 12549583181961831395762060922372139370212061903502107915458762390902627696446;
    
    uint256 constant IC12x = 17284123033835124378431164172844601860315597842516334756357940992818209855085;
    uint256 constant IC12y = 21511582959159984429605098573789708577065186060181366193652399485160019121472;
    
    uint256 constant IC13x = 16484939774507391836809378994989850557977264828515571582063060637691848685903;
    uint256 constant IC13y = 628561376404860980787457079698775999122867899121734070860702384255580692727;
    
    uint256 constant IC14x = 19406106828698285056506153646752039518651463044838145460784758680458966609880;
    uint256 constant IC14y = 20084705027949733002783440715010178623174892142749752081497758455222476824439;
    
    uint256 constant IC15x = 16354088473545672389573123224815233437639059609361106358633388470342385245662;
    uint256 constant IC15y = 4803984906654179281996588405786810939425002205280717488516280607752506503048;
    
    uint256 constant IC16x = 18899003577760713691954340818819502030595767504201043422466093785272119455650;
    uint256 constant IC16y = 1348508532620831200447815892904276846875816165362653455116937036750493237032;
    
 
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
