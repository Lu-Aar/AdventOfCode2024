#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#define LIST_LENGTH 1000

void sort(int* list, int listLength)
{
    bool action = false;
    int buffer = 0;
    do
    {
        action = false;
        for(int ii = 0; ii < listLength-1; ++ii)
        {
            if(list[ii+1] < list[ii])
            {
                action = true;
                
                buffer = list[ii];
                list[ii] = list[ii+1];
                list[ii+1] = buffer;
            }
        }
    }while(action);
}

void CalculateDistance(int* list1, int* list2, int listLength)
{
    printf("Calculate distance...\n\r");
    int distance = 0;
    for(int ii = 0; ii < listLength; ++ii)
    {
        distance += abs(list1[ii] - list2[ii]);
    }
    printf("Distance: %d\n\r", distance);
}

void CalculateSimilarity(int* list1, int* list2, int listLength)
{
    printf("Calculate similarity...\n\r");
    int totalSimilarity = 0;
    int similarity = 0;
    
    for(int ii = 0; ii < listLength; ++ii)
    {
        similarity = 0;
        for(int jj = 0; jj < listLength; ++jj)
        {
            if(list1[ii] == list2[jj])
            {
                ++similarity;
            }
        }
        totalSimilarity += list1[ii] * similarity;
    }
    printf("Similarity: %d", totalSimilarity);
}

int main()
{
    int list1[] = {15131,32438,12503,73808,57168,97870,18072,55097,36615,63626,19535,20386,32817,90111,81180,20278,71822,36650,96658,48953,87910,15586,59112,36552,54311,14521,46924,79287,91525,77523,14229,20514,30019,78834,96186,99772,62659,44811,41778,39724,56672,37872,67848,55337,48137,57761,90770,87830,14118,64511,67927,76032,46116,96284,51398,52363,84966,29725,92179,77151,31670,83560,62543,88635,14456,62848,31081,20287,58199,78634,21931,96294,18891,54875,40666,68475,93112,17780,89025,10487,14442,18287,73030,88998,47512,49438,62202,42062,35797,61308,98706,51049,96082,82102,91642,86709,11536,57804,58098,29840,68500,18840,42529,74868,48556,64424,87547,25349,24698,29610,10024,80179,86009,75396,18309,14754,98264,28294,50417,27171,79907,94597,36251,42293,22552,18765,65758,16028,15096,27990,91867,48177,43833,91992,25767,56422,39281,85701,40915,25378,17827,74880,51583,14996,24393,13557,87102,57922,18436,36961,33558,29752,88141,65521,98636,83809,13839,81470,93818,72210,10853,78121,28208,75892,62655,59696,18410,82521,68047,59444,73400,52751,94229,44728,84688,54885,66708,16610,52212,33467,11933,11426,98122,24974,89866,63042,97878,86324,93550,21893,32422,97593,79956,18574,69417,20527,32901,76892,80784,70030,45455,84089,51290,78084,19446,83281,88850,41737,28334,57007,35230,97142,71663,21967,73266,37751,71909,49256,87879,18958,33674,88578,38184,23162,94689,50188,73498,80946,20434,96199,97269,75164,63802,85851,13886,82608,47631,41955,41722,80210,72032,99588,75340,72016,87488,54539,92646,92493,51219,94294,35339,60625,71044,65428,47057,79756,25694,96457,13870,61984,14480,31924,33548,66463,36209,38146,43109,39326,67802,80246,32312,67288,11591,17019,45637,63060,96782,49139,13278,10622,70609,37494,50253,30770,48235,77045,36713,36647,21913,84907,83140,40049,96240,91375,44851,85957,31141,73469,54379,90253,37361,77965,44088,65748,92167,34050,53845,39439,56254,81736,22481,94515,40865,94090,25243,70472,92113,84122,75877,46967,89199,45284,25894,81546,18098,35845,30136,79351,91114,10796,80157,13776,32867,72659,70346,82707,93643,56487,17461,74189,89473,89648,21185,45756,13958,49766,71014,78218,36683,12805,89940,92619,26109,85199,74790,61077,21011,91299,13145,74616,68242,88099,98816,92627,60619,18592,97961,37971,76808,26960,21159,21820,87396,51739,36150,36428,81368,36684,87943,83934,93369,25759,90295,33748,23965,18136,97136,11751,77584,47535,35157,38666,49584,70553,41660,34861,97329,83411,74970,36340,40870,96019,35996,35750,44648,14216,41202,28670,12646,30412,18596,38836,63846,84175,36177,76440,89469,28950,93707,93976,15262,86652,65454,76119,44343,29424,97543,73497,87093,49551,45191,21283,27141,20255,48823,74136,18194,37794,46775,52309,12317,98269,82747,28972,40842,24862,90115,97189,60429,35563,42174,62129,87307,54431,32208,36159,92813,30956,81881,82389,81627,12133,83315,17677,11309,24911,77700,43948,82304,94870,67575,49305,15751,66060,30935,28356,18811,64019,34225,15964,21017,34116,52676,95338,78527,93795,12924,85065,45073,30072,98834,57372,83929,90423,22832,48430,73629,26263,94074,26414,56951,50559,63550,90368,32146,47438,58449,47368,32574,83798,14115,81573,51307,13174,36828,38526,44179,52115,78802,34661,32323,27132,79104,63502,97485,93125,22160,94150,16960,41717,16797,16286,81133,62615,38433,17558,31164,98740,16053,83861,89754,90249,45631,79425,41085,61288,66018,22664,48544,24552,13795,17199,33038,37670,22237,93548,71344,95400,16457,79172,87249,32398,42920,20491,29343,51134,87045,16859,50451,88013,79869,17158,47399,32349,88683,49926,86432,82719,56739,95877,69037,95747,15660,13684,87807,45695,14570,45507,79826,86233,57051,16227,66940,16621,67736,36049,27324,89099,80234,30105,42497,20275,84842,18545,22010,44172,11085,48341,94313,65788,27317,68318,56329,29327,53630,24518,96364,95503,41188,16708,31294,33264,64245,14497,77398,16427,26395,22620,17321,26035,58951,38225,89267,30825,98399,80743,80557,35024,53184,71001,97339,96913,42578,53465,74719,96923,92916,26229,42662,81620,46956,39197,30330,57702,82062,37754,97560,45988,42413,58786,41724,55364,64004,87772,65614,44781,78305,32629,78238,61460,46112,61748,41222,14622,12228,95242,46228,80525,45245,67491,52166,38396,31358,55034,56827,79835,89347,82227,63832,62004,26506,69651,72433,40513,18717,30405,74331,72157,74767,30847,18242,69756,72005,92985,35084,73020,48804,75034,67280,13826,58014,26965,32785,67828,24669,83563,24440,85814,96689,83424,86597,35057,90254,27503,80689,85142,47144,81029,88306,44736,44675,56611,99455,76675,58779,87090,28546,60401,25679,40839,36746,79132,66132,68819,36538,27205,24399,44240,35819,12508,76682,92401,86766,96643,53030,10426,47697,68686,55276,88787,68667,88551,86242,33643,19228,94076,66464,32020,39310,45805,62915,13011,88834,82701,79163,57767,21571,48453,92533,71231,26769,71828,14891,92462,57405,64689,14673,37392,88004,11897,35121,65978,59214,69160,86199,84182,46746,93280,78233,31751,92129,81082,69123,20505,41538,64819,20294,48547,47633,96892,22150,18701,61229,78415,66928,35556,33671,96468,50427,25039,80962,69045,32104,59924,18240,88763,60348,20717,81946,20510,37273,99008,13046,87961,16856,13289,81110,25077,53154,22799,44901,80985,39889,32167,46469,42664,28717,48319,20405,99160,37312,49441,42459,76352,42818,96351,55426,44989,68109,87896,73110,41918,46805,61826,32096,26386,70847,63265,66807,29422,56355,22577,99518,16441,26818,18649,76731,30590,90428,16076,13553,91615,36190,49749,62226,25075,30720,20085,95853,94343,18518,98111,63474,24642,76558,26475,80306,92346,20270,23131,23638,86655,58258,34945,33250,84692,74810,50681,79610,30424,14983,37491,36576,31400,74498,19311,78393,44638,71239,33836,87014,49752,89676,31538,54931,78700,57624,55476,36922,83133,33766,90922,84184,84873,30986,66208,76128,54499,11089,63966,24165,65766,98485,31557,62423,54888,61829,26964,63722,71060,60721,14154,71805,74964,23705,27306,57026,95790,47055,20026,92097,22924,30454,77910,20644,51589,83392,92902,44609,82287,88814,39082,37969,57693,43012,29944,68317,33738,17794,41165,14871,42771,82391,79484,58189,19422,48923,76839,71377,50558,25393,39296,84189,85258,30912,77881,67451,47082,91778,99760};
    int list2[] = {78158, 35057, 57702, 43128, 71761, 29344, 79079, 92997, 67927, 85851, 91599, 53482, 55364, 84813, 13958, 82963, 70030, 74064, 93643, 12223, 32104, 46936, 47057, 32104, 15545, 35085, 41818, 17659, 46171, 63000, 61327, 77151, 35029, 57168, 92129, 60459, 23209, 97907, 46924, 54539, 11103, 44088, 46924, 58014, 34267, 47261, 82701, 32104, 43916, 97016, 26386, 55621, 43948, 26386, 32104, 45099, 24105, 47057, 73851, 57702, 23131, 34930, 62129, 55364, 87507, 35057, 15535, 11590, 55876, 63263, 62559, 23071, 21157, 35057, 75460, 71547, 57702, 50710, 82701, 77363, 67828, 10668, 47436, 81809, 43866, 52980, 13958, 33002, 54746, 74313, 85679, 29787, 47057, 77395, 37908, 16527, 39339, 77151, 58014, 37272, 77570, 17486, 62129, 62855, 44088, 35057, 70030, 55364, 95115, 37403, 93643, 44088, 67277, 28216, 37500, 67927, 13958, 77151, 96240, 12050, 32187, 73714, 58014, 43948, 46924, 85646, 37771, 74814, 32104, 62738, 23131, 62129, 67927, 96240, 58014, 35328, 87024, 70030, 95206, 23022, 44745, 31126, 56967, 82701, 64040, 31621, 77151, 50349, 82701, 68507, 18947, 24528, 99508, 66940, 24754, 23131, 85851, 35057, 69550, 82295, 84284, 55200, 85199, 13111, 18720, 86838, 26693, 30547, 85199, 62129, 82753, 25250, 67927, 45424, 47457, 53247, 85987, 19000, 47057, 90494, 34797, 23370, 11318, 98740, 61735, 84601, 62129, 88329, 65587, 32872, 82701, 95724, 43017, 66940, 70030, 62986, 78479, 14216, 20119, 46924, 19210, 55364, 32104, 35819, 43664, 82701, 60032, 26655, 87060, 63359, 79052, 62408, 44088, 56111, 70030, 73992, 10908, 76543, 10005, 99715, 26386, 16115, 92129, 94507, 27138, 23236, 92740, 14521, 62129, 54539, 74914, 56409, 46857, 39471, 21128, 43875, 35057, 83525, 49739, 73310, 29229, 19663, 68873, 64274, 54539, 89609, 49227, 92129, 62129, 66940, 82701, 25516, 38554, 57974, 38596, 64175, 69619, 13207, 77151, 33464, 55364, 14216, 35819, 52868, 21830, 62515, 93643, 73266, 83861, 93894, 70030, 20405, 84936, 49240, 58014, 82442, 87050, 57325, 38425, 58014, 55364, 57385, 77151, 88060, 27041, 46924, 60284, 73266, 37190, 76623, 58014, 38908, 49588, 20178, 53224, 77812, 62129, 75003, 73141, 26386, 83861, 86183, 98740, 44088, 27474, 39710, 35057, 70030, 80666, 35057, 41619, 47057, 47438, 94307, 99699, 44088, 36000, 44088, 47057, 98088, 62129, 61031, 24258, 49603, 62129, 58264, 84643, 85199, 29608, 73266, 13958, 34799, 82701, 83316, 32104, 26071, 88453, 73266, 80601, 35819, 80231, 29082, 91451, 24922, 92570, 46924, 15764, 47624, 52276, 18935, 75569, 62002, 85199, 47057, 77094, 44984, 24325, 35819, 92130, 25253, 57168, 29422, 50970, 32104, 54539, 99696, 85199, 86471, 65290, 44025, 18065, 51610, 13958, 69215, 67927, 77167, 85546, 74331, 57127, 39052, 57168, 23805, 85638, 35057, 56026, 47566, 87779, 98740, 72651, 14216, 40935, 20886, 34431, 57619, 90498, 70030, 82739, 32104, 72228, 26425, 92129, 35057, 62535, 17289, 26386, 68378, 76995, 44197, 99599, 91461, 74757, 73266, 70555, 23131, 25567, 41764, 30895, 43948, 70030, 73909, 31486, 23131, 88787, 46924, 98685, 46693, 32104, 76030, 58014, 16528, 73266, 34339, 73266, 57702, 35838, 57964, 80112, 96240, 98740, 42291, 39213, 96240, 89172, 54579, 85851, 46924, 29514, 85851, 85233, 99130, 69145, 12281, 72432, 47057, 43948, 92837, 16218, 87764, 16847, 14216, 46027, 59074, 88230, 82701, 32104, 35927, 98147, 53055, 22718, 55364, 85199, 93182, 15044, 26171, 41610, 32565, 12256, 23131, 17638, 13958, 44088, 81436, 32857, 16079, 46924, 85851, 64525, 79359, 47356, 43948, 32104, 73266, 79432, 82701, 98740, 59406, 93643, 94506, 14216, 98740, 55364, 43948, 31546, 63317, 46924, 57185, 43948, 85851, 15690, 92129, 48770, 66940, 52219, 47057, 67828, 44088, 54539, 89989, 62495, 14805, 59466, 30442, 63409, 46386, 55364, 45846, 14216, 21543, 75856, 62129, 13241, 31406, 73266, 36776, 47057, 77268, 99409, 37183, 46924, 14216, 14216, 14216, 44427, 67927, 82701, 93605, 98413, 49595, 78826, 57029, 59246, 69353, 93573, 50912, 40613, 92536, 90698, 97202, 29422, 12325, 25678, 57782, 65021, 17593, 15560, 56929, 18964, 67927, 23131, 99947, 26386, 13958, 93686, 20008, 69537, 93643, 89768, 35057, 92864, 43633, 59576, 72337, 23131, 67506, 24977, 44088, 23131, 96958, 46189, 96377, 96244, 70718, 87229, 77709, 19049, 43120, 13585, 46924, 73266, 82570, 85199, 47057, 67828, 31766, 89576, 33974, 32612, 28004, 14216, 46924, 92981, 70030, 60704, 91745, 57702, 82701, 81945, 47057, 40000, 20405, 30878, 98085, 96240, 26073, 21420, 72248, 46924, 93997, 12686, 72607, 54463, 30571, 35057, 66365, 67169, 46924, 91342, 72939, 15700, 47443, 33591, 78805, 73266, 82681, 75723, 58014, 71872, 14521, 96240, 47057, 29400, 98476, 45605, 20507, 62129, 10380, 40495, 54539, 38989, 26969, 60576, 69527, 44088, 31704, 23131, 53388, 40217, 36036, 51508, 14216, 96240, 90420, 62129, 23131, 54539, 60673, 69114, 75711, 46098, 58014, 83687, 67828, 43948, 37772, 46924, 93798, 58014, 18043, 32104, 10083, 92531, 29640, 82716, 63891, 92129, 23131, 44088, 67849, 54539, 35819, 20586, 67844, 57702, 86192, 52372, 92992, 93643, 57702, 55364, 44088, 28935, 41912, 70621, 88974, 12565, 46791, 98740, 99882, 89250, 99433, 92129, 19703, 75605, 45668, 57779, 43542, 32104, 55196, 85851, 91531, 70030, 75708, 48355, 46015, 43948, 93643, 78920, 31361, 91728, 57168, 43948, 82701, 61087, 36117, 12235, 76760, 23139, 45473, 96240, 99554, 43948, 85296, 41706, 27209, 85851, 29096, 20405, 82701, 98740, 14521, 54484, 82701, 93860, 17327, 49220, 63820, 95977, 57329, 47057, 82325, 96018, 60296, 40986, 44460, 37444, 47909, 56724, 66228, 20251, 43948, 54336, 96240, 30317, 74559, 62129, 45242, 75437, 12408, 68886, 85851, 19747, 46924, 35819, 82701, 78031, 79283, 70030, 31194, 42061, 44088, 41105, 44088, 85851, 66320, 43948, 28581, 73266, 31547, 15882, 58776, 41774, 93643, 42035, 32104, 65236, 79431, 62129, 87478, 58131, 99100, 32104, 62129, 75645, 23131, 70030, 58014, 43980, 51159, 79855, 68239, 43948, 54986, 53592, 14216, 87339, 29653, 65830, 66583, 96594, 77151, 98740, 40597, 66837, 67057, 19487, 77151, 67927, 23742, 63271, 68742, 86445, 84577, 77403, 25777, 73266, 89012, 69362, 93643, 67066, 81830, 89259, 35819, 73266, 43948, 54757, 22428, 17131, 63742, 18158, 36229, 37525, 46924, 44088, 66929, 76171, 85851, 61187, 91024, 58160, 67927, 37915, 32416, 63670, 49989, 58313, 46700, 89560, 28489, 74174, 35819, 80833, 86331, 33979, 67927, 23131, 58014, 58014, 48193, 95402, 43948, 60254, 57352, 45523, 61572, 96240, 11925, 80726, 52269, 73266, 17447, 82701, 93643, 73266, 92479, 62129, 85090, 54666, 35057, 35819, 77370, 35057, 31956, 85199, 14521, 18949, 85199, 83620, 42249, 23131, 96903, 38715, 92129, 85199, 98740, 46301, 66981, 44088, 46924, 73266, 66921, 60756, 13958, 92129, 32104, 44088, 52459, 75842, 63459, 32104, 45395, 85952, 56940, 82701, 80051, 85348, 30788, 35057, 30087, 67481, 75622, 12792, 38758, 42427, 99903, 30998, 39385, 76104, 93643, 42484, 20771, 15393, 73266, 85199, 96633, 93643, 78876, 66940, 58014, 98740, 47180, 41373, 43164, 29055, 53019, 29804, 82701, 83734, 85851, 23080, 44088, 45650, 84954, 57168, 22386, 13039, 48653, 46924, 89946, 85851, 37363};

    printf("Sort list1\n\r");
    sort(list1, LIST_LENGTH);
    
    printf("Sort list2\n\r");
    sort(list2, LIST_LENGTH);
    
    CalculateDistance(list1, list2, LIST_LENGTH);
    CalculateSimilarity(list1, list2, LIST_LENGTH);
    
    return 0;
}