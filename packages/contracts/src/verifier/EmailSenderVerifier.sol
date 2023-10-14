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
    uint256 constant deltax1 = 9221393175182623932527504233880971206336390090228981284086022731330204807812;
    uint256 constant deltax2 = 2574516337446816812527284520122140316218416702316069272559778485852392381368;
    uint256 constant deltay1 = 3956769475140231711873005402084363658572670878203428470733838637734528971389;
    uint256 constant deltay2 = 4858768646839459647112558841668531014832957177429290945451200379238850384632;

    
    uint256 constant IC0x = 5174799347889354358179247479120410331850885496479447395919959330986984457792;
    uint256 constant IC0y = 8501958193101554410704207546151122966994359755603206464890475803813122678042;
    
    uint256 constant IC1x = 15980652270608435169941169886860200511684122811786483286688369301427654555287;
    uint256 constant IC1y = 7165725735135900010482246868231092329850420282450321829121401736212012314106;
    
    uint256 constant IC2x = 19687272789410136543442617070705755347281793250352302782552749318472467816996;
    uint256 constant IC2y = 21849200519134603495263138454091449267059764027792140749156385451541093897757;
    
    uint256 constant IC3x = 5717349418310881093216925434997213819728299104221208200779750866688700211852;
    uint256 constant IC3y = 5874252271859902291276010773463268244636934130378425003494022626205085826805;
    
    uint256 constant IC4x = 20316948583230553255556265487659535720355536835727502190546984161375927744589;
    uint256 constant IC4y = 9548473010616558410150161649283378619725223964470243248873362837097987641604;
    
    uint256 constant IC5x = 961323213876770443956366374518782806676199478491878544706794896571928072928;
    uint256 constant IC5y = 7203985524668883110631444212644852630515906774827817666367557096584471603292;
    
    uint256 constant IC6x = 4114056787094613288684482252735314560600542272389470726305480110242320396357;
    uint256 constant IC6y = 15781299129635041520202223770111392996131200546794518829202822128930050855549;
    
    uint256 constant IC7x = 9870997818338543367202440042444400810341658140380218209132015568815899435422;
    uint256 constant IC7y = 2438592787012916206243240780677463754486174982028606593999195032406469682692;
    
    uint256 constant IC8x = 2234078500733913656719984425912276753060694973894669034234829319217737663901;
    uint256 constant IC8y = 19806765024636550323534188428364157822552279920375377013397771597532933679371;
    
    uint256 constant IC9x = 7276577670988736701390088099324812288014571371366746080431323796132903744596;
    uint256 constant IC9y = 9498086338649690957077747236354001359091540378800106373032382756444131408614;
    
    uint256 constant IC10x = 991605899484854803962998597769674966697060864785187678538497205283351599366;
    uint256 constant IC10y = 17704075102695425317222198594775270503488042859147607674845492337394090570637;
    
    uint256 constant IC11x = 16182187725796555673702414113891740755015237603107515215983907626237952445613;
    uint256 constant IC11y = 19558241751958172182285769916839237871763210411552962817779953810521099291968;
    
    uint256 constant IC12x = 18524284046949075551939645806606361505620778517792724385868373089399778823691;
    uint256 constant IC12y = 18282375469437704047359488710320365151404976358933696909698086179622497106068;
    
    uint256 constant IC13x = 4455004275968298343828639560207865297424739513792030223253266402208905806227;
    uint256 constant IC13y = 14681375116914070183535889788518861022010986893046652748451374082316480903080;
    
    uint256 constant IC14x = 15358626403446813917938848157064206288178316592342337814176725603433050470123;
    uint256 constant IC14y = 8916266577119849395814195998460875102458177760861062410429832711173787451299;
    
    uint256 constant IC15x = 7052105520664678591545192563589850983745620094238210031260261676855604069299;
    uint256 constant IC15y = 4271576011622040093126320238114104220945210708420359687699677330065482812480;
    
    uint256 constant IC16x = 20619249436500080326597132960310479654229552709099677648798717846843815026448;
    uint256 constant IC16y = 6531960191484732819088079571877719578754584179663905351586230493426463759980;
    
    uint256 constant IC17x = 393920070152336056089226670054679427486281141602736385450183448066272929151;
    uint256 constant IC17y = 49275514335051916177939786155275186654324295028309413724786914122938712835;
    
    uint256 constant IC18x = 20015908858506459808594086730484325784823504612521257497160732020806311589972;
    uint256 constant IC18y = 21758871428317337162117881162496615125228090755489429158177363323616254889008;
    
    uint256 constant IC19x = 228156860947568601053870380600134556988930404836537622010483864760333352738;
    uint256 constant IC19y = 17438117452220864589837150241066469540398817488416766094529697638306614218599;
    
    uint256 constant IC20x = 11193470940749257823112691044142234081437671836406772442661836780166925661455;
    uint256 constant IC20y = 6272190457098541001335627465020000797929049486349065699449620543062396988526;
    
    uint256 constant IC21x = 20102166604348719973924715144336571306033585934214729256567216125090082251166;
    uint256 constant IC21y = 17712900949759367154192039267336256064565909695079076036014497641085586670486;
    
    uint256 constant IC22x = 21779945713149972195850123357654697123450356561906985417391906472742816407733;
    uint256 constant IC22y = 7082883082745526273667387635189345779076115000515726486212332688230000970508;
    
    uint256 constant IC23x = 13895932253104500947523142832545215848558744591799187359897492586660517469284;
    uint256 constant IC23y = 6108966558477040271096328986643909686990189244414358652831321613577600749333;
    
    uint256 constant IC24x = 3677971014592376142642497296662367335985964777527371950970457519773845774544;
    uint256 constant IC24y = 430636439171039958908766951057229777614446705855630595007734081023673262901;
    
    uint256 constant IC25x = 10905232225222792372658480630660914702868778837910161378051017249653791605803;
    uint256 constant IC25y = 6775475181025617565759442250578445776713799333732538567506105415677585810006;
    
    uint256 constant IC26x = 21047231123101839451202504060278946896655613106755654569551222715373806081879;
    uint256 constant IC26y = 14135095694788174353483393606396329792614850725974831413597931095497031422284;
    
    uint256 constant IC27x = 17361197371519186227137534817693367968106373941859108690409861514109155897232;
    uint256 constant IC27y = 7828700772870920627914351450153962697024737456953804757465829313851312107773;
    
    uint256 constant IC28x = 3714466894819763541183521519245788632305078093008561880760082811895536289521;
    uint256 constant IC28y = 5216132341911431158753604331940722635675975211867231204133017781579876268639;
    
    uint256 constant IC29x = 15818161248865347582491600956082512834515219808224616746561896166953010927680;
    uint256 constant IC29y = 12605086161315300667861761087618653346840635223661832539409357833081136803355;
    
    uint256 constant IC30x = 19112146556273023920293761940496035150190786021007672492598211784987850329964;
    uint256 constant IC30y = 12769532238980690692752224063708735805506799000025242112178570621347455243061;
    
    uint256 constant IC31x = 9799877334956418122938811941363989967604203173146880843508381755788085056520;
    uint256 constant IC31y = 6225430857639143818462503596216405569045792922818428301102342294369142709191;
    
    uint256 constant IC32x = 19229684562900922054901742943553619208873495823200670678298958564689570422948;
    uint256 constant IC32y = 5260907844503930980648135348565599070931164555965843236683704948515536969570;
    
    uint256 constant IC33x = 10795486839862717627275460823615561434142783755275743487295762229126075150289;
    uint256 constant IC33y = 17048760938971630924478335751359599291886343083877166798865489687502673198937;
    
 
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
