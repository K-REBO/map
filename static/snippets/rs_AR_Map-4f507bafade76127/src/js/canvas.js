let magnification = window.innerHeight / 270;
let debug_mode = false;
let canvas_width = 500 * magnification;
let canvas_height = 270 * magnification;
let board;
let ctx;
let BaseFillColor = "#6B7280"


const map = 
{
	"map1":{
		"mapName" : "1F Map",
		"isLineWidthConf": false,
		"isNodeSizeConf":false,
		"line": [
			[
				[38,25],[195,25],[195,66],[303,66],[303,113],[375,113],[375,5],[490,5]
			],
			[
				[38,25],[38,66],[195,66]
			],
			[
				[90,25],[90,171]
			],
			[
				[38,66],[38,171]
			],
			[
				[38,124],[90,124]
			],
			[
				[38,148],[90,148]
			],
			[
				[63,148],[63,171]
			],
			[
				[38,171],[90,171]
			],
			[
				[116,25],[116,66]
			],
			[
				[143,25],[143,66]
			],
			[
				[104,84],[104,171],[285,171],[285,84],[104,84]
			],
			[
				[303,240],[303,193],[251,193],[251,188],[38,188],[38,240],[303,240]
			],
			[
				[78,188],[78,240]
			],
			[
				[104,188],[104,240]
			],
			[
				[157,188],[157,240]
			],
			[
				[184,188],[184,240]
			],
			[
				[251,188],[251,240]
			],
			[
				[396,219],[435,219],[435,254],[303,254],[303,128],[396,128],[396,254]
			],
			[
				[90,35],[116,35]
			],
			[
				[90,45],[116,45]
			],
			[
				[90,55],[116,55]
			],
			[
				[490,5],[490,113],[460,113],[460,66]
			],
			[
				[470,113],[470,171]
			],
			[
				[412,219],[412,171],[490,171],[490,219],[435,219]
			],
			[
				[375,113],[420,113],[420,66],[375,66]
			],
			[
				[375,97],[420,97]
			],
			[
				[375,80],[420,80]
			],
			[
				[410,66],[410,113]
			],
			[
				[400,66],[400,113]
			],
			[
				[390,66],[390,113]
			],
			[
				[383,66],[383,113]
			],
			[
				[143,84],[143,124]
			],
			[
				[217,84],[217,124]
			],
			[
				[233,84],[233,124]
			],
			[
				[104,124],[285,124]
			],
			[
				[303,66],[320,66],[320,113]
			],
			[
				[320,80],[303,80]
			],
			[
				[303,97],[320,97]
			],
			[
				[285,66],[285,50],[303,50],[303,66]
			],
			[
				[277,193],[277,240]
			],
			[
				[169,25],[169,66]
			],
			[
				[400,200],
			]
		],
	
		"place": [
			{
				"placeName":"厨房",
				"pos":[43,52],
				"fontSize":130,
			},
			{
				"placeName":"トイレ",
				"pos":[114,44],
				"fontSize":60,
			},
			{
				"placeName":"男",
				"pos":[123,54],
				"fontSize":70,
			},
			{
				"placeName":"トイレ",
				"pos":[141,44],
				"fontSize":60,
			},
			{
				"placeName":"女",
				"pos":[150,54],
				"fontSize":70,
			},
			{
				"placeName":"湯沸室",
				"pos":[169,48],
				"fontSize":55,
			},				{
				"placeName":"生徒相談室",
				"pos":[38,139],
				"fontSize":65,
			},				
			{
				"placeName":"調整室",
				"pos":[38,163],
				"fontSize":54,
			},
			{
				"placeName":"放送室",
				"pos":[64,163],
				"fontSize":55,
			},
			{
				"placeName":"情報準備室",
				"pos":[105,108],
				"fontSize":48,
			},
			{
				"placeName":"コンピュータ室",
				"pos":[144,108],
				"fontSize":65,
			},
			{
				"placeName":"談話室",
				"pos":[218,108],
				"fontSize":30,
			},
			{
				"placeName":"定時制",
				"pos":[238,99],
				"fontSize":80,
			},
			{
				"placeName":"執務室",
				"pos":[238,115],
				"fontSize":80,
			},
			{
				"placeName":"中庭",
				"pos":[160,160],
				"fontSize":190,
			},
			{
				"placeName":"保健室",
				"pos":[38,220],
				"fontSize":80,
			},
			{
				"placeName":"書道室",
				"pos":[105,220],
				"fontSize":100,
			},
			{
				"placeName":"書道",
				"pos":[80,215],
				"fontSize":70,
			},
			{
				"placeName":"準備室",
				"pos":[78,230],
				"fontSize":55,
			},
			{
				"placeName":"美術室",
				"pos":[187,220],
				"fontSize":120,
			},
			{
				"placeName":"美術",
				"pos":[160,215],
				"fontSize":70,
			},
			{
				"placeName":"準備室",
				"pos":[157,230],
				"fontSize":55,
			},
			{
				"placeName":"第二",
				"pos":[255,215],
				"fontSize":45,
			},
			{
				"placeName":"会議室",
				"pos":[252,225],
				"fontSize":45,
			},
			{
				"placeName":"応接室",
				"pos":[280,220],
				"fontSize":45,
			},
			{
				"placeName":"生徒",
				"pos":[332,180],
				"fontSize":100,
			},
			{
				"placeName":"ホール",
				"pos":[325,200],
				"fontSize":100,
			},
			{
				"placeName":"トイレ男",
				"pos":[320,75],
				"fontSize":55,
			},
			{
				"placeName":"トイレバリアフリー",
				"pos":[320,92],
				"fontSize":38,
			},
			{
				"placeName":"トイレ女",
				"pos":[320,108],
				"fontSize":55,
			},
			{
				"placeName":"校長室",
				"pos":[398,240],
				"fontSize":70,
			},
			{
				"placeName":"事務室",
				"pos":[425,200],
				"fontSize":100,
			},
			{
				"placeName":"玄関",
				"pos":[444,144],
				"fontSize":70,
			},
			{
				"placeName":"生徒昇降口",
				"pos":[395,40],
				"fontSize":100,
			},
			{
				"placeName":"食堂",
				"pos":[47,100],
				"fontSize":100,
			}
		],
	
		"Node" : [
			{
				"NodeID":1,
				"pos":[97,75],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":2,
				"pos":[100,50],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":3,
				"pos":[130,75],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":4,
				"pos":[155,75],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":5,
				"pos":[183,75],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":6,
				"pos":[225,75],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":7,
				"pos":[265,75],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":8,
				"pos":[295,75],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":9,
				"pos":[295,89],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":28,
				"pos":[295,105],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":10,
				"pos":[295,121],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":19,
				"pos":[350,121],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":27,
				"pos":[350,180],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":20,
				"pos":[440,121],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":21,
				"pos":[440,88],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":24,
				"pos":[440,160],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":25,
				"pos":[405,160],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":26,
				"pos":[405,210],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":23,
				"pos":[400,88],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":22,
				"pos":[440,40],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":11,
				"pos":[295,180],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":12,
				"pos":[230,180],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":13,
				"pos":[170,180],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":14,
				"pos":[130,180],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":15,
				"pos":[97,180],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":17,
				"pos":[97,160],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":18,
				"pos":[97,140],
				"radius":3.5,
				"colors":"#f2f0ea"
			},
			{
				"NodeID":16,
				"pos":[48,180],
				"radius":3.5,
				"colors":"#f2f0ea"
			}
		],
		"Rect" :[
			{
				"pos" : [],
				"color" : "#00ff00",
			},
			{
				"pos" : [],
				"color" : "#00ff00",
			}
		],

	},
	"map2":
	{
		"mapName" : "2F Map",
		"isLineWidthConf" : false,
		"isNodeSizeConf" : false,
		"line" : [
			[
				[90,25],[169,25],[169,66],[303,66],[303,113],[375,113],[375,5],[490,5]
			],
			[
				[450,140],[490,140]
			],
			[
				[20,66],[195,66]
			],
			[
				[90,25],[90,66]
			],
			[
				[50,124],[50,84],[90,84],[90,171]
			],
			[
				[70,66],[70,124]
			],
			[
				[20,66],[20,124]
			],
			[
				[20,124],[90,124]
			],
			[
				[20,171],[90,171]
			],
			[
				[116,25],[116,66]
			],
			[
				[143,25],[143,66]
			],
			[
				[104,84],[104,171],[285,171],[285,84],[104,84]
			],
			[
				[303,188],[251,188],[251,188],[20,188],[20,240],[303,240]
			],
			[
				[78,188],[78,240]
			],
			[
				[120,188],[120,240]
			],
			[
				[180,188],[180,240]
			],
			[
				[200,188],[200,240]
			],
			[
				[396,254],[396,128],[303,128],[303,254],[460,254]
			],
			[
				[90,35],[116,35]
			],
			[
				[90,45],[116,45]
			],
			[
				[90,55],[116,55]
			],
			[
				[490,5],[490,113],[440,113],[440,5]
			],
			[
				[490,113],[490,190],[455,190],[455,140],[440,140],[440,113]
			],
			[
				[490,190],[440,190],[440,220],[490,220]
			],
			[
				[375,113],[420,113],[420,66],[375,66]
			],
			[
				[375,97],[420,97]
			],
			[
				[375,80],[420,80]
			],
			[
				[410,66],[410,113]
			],
			[
				[400,66],[400,113]
			],
			[
				[390,66],[390,113]
			],
			[
				[383,66],[383,113]
			],
			[
				[130,84],[130,124]
			],
			[
				[200,84],[200,124]
			],
			[
				[240,84],[240,124]
			],
			[
				[104,124],[285,124]
			],
			[
				[303,66],[320,66],[320,113]
			],
			[
				[320,80],[303,80]
			],
			[
				[303,97],[320,97]
			],
			[
				[285,66],[285,50],[303,50],[303,66]
			],
			[
				[440,50],[375,50]
			],
			[
				[420,50],[420,70]
			],
			[
				[440,35],[490,35]
			],
			[
				[452,60],[490,60]
			],
			[
				[440,75],[452,75]
			],
			[
				[452,35],[452,75]
			],
			[
				[460,254],[460,245],[490,245]
			],
			[
				[104,140],[120,140],[120,160],[104,160]
			],
			[
				[109,140],[109,160]
			],
			[
				[115,140],[115,160]
			]
		],
		"place" : [
			{
				"placeName":"トイレ",
				"pos":[120,50],
				"fontSize":80,
			},
			{
				"placeName":"地学教室",
				"pos":[25,80],
				"fontSize":60,
			
			},
			{
				"placeName":"地研",
				"pos":[50,105],
				"fontSize":60,
			
			},
			{
				"placeName":"家庭科教室",
				"pos":[140,105],
				"fontSize":60,
			
			},
			{
				"placeName":"家準",
				"pos":[107,105],
				"fontSize":50,
			
			},
			{
				"placeName":"国研",
				"pos":[210,105],
				"fontSize":60,
			},
			{
				"placeName":"数研",
				"pos":[250,105],
				"fontSize":60,
			
			},
			{
				"placeName":"調理室",
				"pos":[30,220],
				"fontSize":80,
			
			},
			{
				"placeName":"進路",
				"pos":[90,210],
				"fontSize":60,
			
			},
			{
				"placeName":"指導室",
				"pos":[85,220],
				"fontSize":60,
			
			},
			{
				"placeName":"国際科",
				"pos":[135,210],
				"fontSize":60,
			
			},
			{
				"placeName":"教務室",
				"pos":[135,220],
				"fontSize":60,
			
			},
			{
				"placeName":"休養室",
				"pos":[180,215],
				"fontSize":40,
			
			},
			{
				"placeName":"教務室",
				"pos":[230,215],
				"fontSize":70,
			
			},
			{
				"placeName":"女子休養室",
				"pos":[376,60],
				"fontSize":50,
			
			},
			{
				"placeName":"視聴覚",
				"pos":[390,30],
				"fontSize":60,
			
			},
			{
				"placeName":"英大",
				"pos":[455,23],
				"fontSize":50,
			
			},
			{
				"placeName":"英小",
				"pos":[460,50],
				"fontSize":50,
			
			},
			{
				"placeName":"英研",
				"pos":[450,100],
				"fontSize":60,
			
			},
			{
				"placeName":"生徒会室",
				"pos":[445,130],
				"fontSize":60,
			
			},
			{
				"placeName":"社研",
				"pos":[460,160],
				"fontSize":60,
			
			},
		],
		"Node" : [
			{
				"NodeID":1,
				"pos":[80,75],
				"radius":3.5,
				"colors":"red"
			},
			{
				"NodeID":2,
				"pos":[97,75],
				"radius":3.5,
				"colors":"red"
			},
			{
				"NodeID":3,
				"pos":[130,75],
				"radius":3.5,
				"colors":"red"
			},
			{
				"NodeID":4,
				"pos":[155,75],
				"radius":3.5,
				"colors":"red"
			},
			{
				"NodeID":5,
				"pos":[220,75],
				"radius":3.5,
				"colors":"red"
			},
			{
				"NodeID":6,
				"pos":[260,75],
				"radius":3.5,
				"colors":"red"
			},
			{
				"NodeID":7,
				"pos":[295,75],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":8,
				"pos":[295,100],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":9,
				"pos":[295,178],
				"radius":3.5,
				"clolrs":"red",
			},				
			{
				"NodeID":10,
				"pos":[190,178],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":11,
				"pos":[150,178],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":12,
				"pos":[97,178],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":13,
				"pos":[60,178],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":14,
				"pos":[97,145],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":15,
				"pos":[430,123],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":16,
				"pos":[430,90],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":17,
				"pos":[430,60],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":18,
				"pos":[445,60],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":19,
				"pos":[445,40],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":20,
				"pos":[430,180],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":21,
				"pos":[430,230],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":22,
				"pos":[295,123],
				"radius":3.5,
				"clolrs":"red",
			},
			{
				"NodeID":23,
				"pos":[97,60],
				"radius":3.5,
				"clolrs":"red",
			}
		],
		"Rect" :[
			{
				"pos" : [],
				"color" : "#00ff00",
			},
			{
				"pos" : [],
				"color" : "#00ff00",
			}
		],
	},
	"map3":{
		"mapName" : "2F Map",
		"isLineWidthConf" : false,
		"isNodeSizeConf" : false,
		"line" : [
			[
				[20,120],[320,120],[320,160],[410,160],[410,90],[350,90],[350,30],[488,30],[488,225],[280,225],[280,180],[20,180],[20,120]
			],
			[
				[20,135],[280,135],[280,180]
			],
			[
				[280,180],[488,180]
			],
			[
				[120,120],[120,80],[170,80],[170,120]
			],
			[
				[65,135],[65,180]
			],
			[
				[105,135],[105,180]
			],
			[
				[150,135],[150,180]
			],
			[
				[195,135],[195,180]
			],
			[
				[240,135],[240,180]
			],
			[
				[330,180],[330,225]
			],
			[
				[383,180],[383,225]
			],
			[
				[435,180],[435,225]
			],
			[
				[488,160],[435,160],[435,30]
			],
			[
				[410,90],[410,30]
			]
		],
		"place" : [
			{
				"placeName":"トイレ",
				"pos":[140,100],
				"fontSize":60,
			
			},
			{
				"placeName":"31HR",
				"pos":[30,160],
				"fontSize":70,
			
			},
			{
				"placeName":"32HR",
				"pos":[72,160],
				"fontSize":70,
			
			},
			{
				"placeName":"33HR",
				"pos":[112,160],
				"fontSize":70,
			
			},
			{
				"placeName":"34HR",
				"pos":[156,160],
				"fontSize":70,
			
			},
			{
				"placeName":"35HR",
				"pos":[200,160],
				"fontSize":70,
			
			},
			{
				"placeName":"36HR",
				"pos":[245,160],
				"fontSize":70,
			
			},
			{
				"placeName":"37HR",
				"pos":[289,205],
				"fontSize":70,
			
			},
			{
				"placeName":"38HR",
				"pos":[340,205],
				"fontSize":70,
			
			},
			{
				"placeName":"39HR",
				"pos":[393,205],
				"fontSize":70,
			
			},
			{
				"placeName":"30HR",
				"pos":[445,205],
				"fontSize":70,
			
			},


		],
		"Node" : [
			{
				"NodeID":1,
				"pos":[128,110],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":2,
				"pos":[50,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":3,
				"pos":[100,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":4,
				"pos":[128,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":5,
				"pos":[145,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":6,
				"pos":[165,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":7,
				"pos":[210,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":8,
				"pos":[255,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":9,
				"pos":[300,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":10,
				"pos":[300,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":11,
				"pos":[350,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":12,
				"pos":[423,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":13,
				"pos":[450,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":14,
				"pos":[423,145],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":15,
				"pos":[423,130],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":16,
				"pos":[423,100],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":17,
				"pos":[423,80],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":18,
				"pos":[400,130],
				"radius":3.5,
				"colors":""
			}
		],
		"Rect" :[
			{
				"pos" : [],
				"color" : "#00ff00",
			},
			{
				"pos" : [],
				"color" : "#00ff00",
			}
		],

	},
	"map4":{
		"mapName" : "2F Map",
		"isLineWidthConf" : false,
		"isNodeSizeConf" : false,
		"line" : [
			[
				[20,120],[320,120],[320,160],[410,160],[410,90],[350,90],[350,30],[488,30],[488,225],[280,225],[280,180],[20,180],[20,120]
			],
			[
				[20,135],[280,135],[280,180]
			],
			[
				[280,180],[488,180]
			],
			[
				[120,120],[120,80],[170,80],[170,120]
			],
			[
				[65,135],[65,180]
			],
			[
				[105,135],[105,180]
			],
			[
				[150,135],[150,180]
			],
			[
				[195,135],[195,180]
			],
			[
				[240,135],[240,180]
			],
			[
				[330,180],[330,225]
			],
			[
				[383,180],[383,225]
			],
			[
				[435,180],[435,225]
			],
			[
				[488,160],[435,160],[435,30]
			],
			[
				[410,90],[410,30]
			]
		],	
		"place" : [
			{
				"placeName":"トイレ",
				"pos":[140,100],
				"fontSize":60,
			
			},
			{
				"placeName":"21HR",
				"pos":[30,160],
				"fontSize":70,
			
			},
			{
				"placeName":"22HR",
				"pos":[72,160],
				"fontSize":70,
			
			},
			{
				"placeName":"23HR",
				"pos":[112,160],
				"fontSize":70,
			
			},
			{
				"placeName":"24HR",
				"pos":[156,160],
				"fontSize":70,
			
			},
			{
				"placeName":"25HR",
				"pos":[200,160],
				"fontSize":70,
			
			},
			{
				"placeName":"26HR",
				"pos":[245,160],
				"fontSize":70,
			
			},
			{
				"placeName":"27HR",
				"pos":[289,205],
				"fontSize":70,
			
			},
			{
				"placeName":"28HR",
				"pos":[340,205],
				"fontSize":70,
			
			},
			{
				"placeName":"29HR",
				"pos":[393,205],
				"fontSize":70,
			
			},
			{
				"placeName":"20HR",
				"pos":[445,205],
				"fontSize":70,
			
			},

		],
		"Node" : [
			{
				"NodeID":1,
				"pos":[128,110],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":2,
				"pos":[50,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":3,
				"pos":[100,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":4,
				"pos":[128,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":5,
				"pos":[145,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":6,
				"pos":[165,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":7,
				"pos":[210,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":8,
				"pos":[255,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":9,
				"pos":[300,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":10,
				"pos":[300,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":11,
				"pos":[350,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":12,
				"pos":[423,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":13,
				"pos":[450,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":14,
				"pos":[423,145],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":15,
				"pos":[423,130],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":16,
				"pos":[423,100],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":17,
				"pos":[423,80],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":18,
				"pos":[400,130],
				"radius":3.5,
				"colors":""
			},

		],
		"Rect" :[
			{
				"pos" : [],
				"color" : "#00ff00",
			},
			{
				"pos" : [],
				"color" : "#00ff00",
			}
		],

	},
	"map5": {
		"mapName" : "2F Map",
		"isLineWidthConf" : false,
		"isNodeSizeConf" : false,
		"line" : [
			[
				[20,120],[320,120],[320,160],[410,160],[410,90],[350,90],[350,30],[488,30],[488,225],[280,225],[280,180],[20,180],[20,120]
			],
			[
				[20,135],[280,135],[280,180]
			],
			[
				[280,180],[488,180]
			],
			[
				[120,120],[120,80],[170,80],[170,120]
			],
			[
				[65,135],[65,180]
			],
			[
				[105,135],[105,180]
			],
			[
				[150,135],[150,180]
			],
			[
				[195,135],[195,180]
			],
			[
				[240,135],[240,180]
			],
			[
				[330,180],[330,225]
			],
			[
				[383,180],[383,225]
			],
			[
				[435,180],[435,225]
			],
			[
				[488,160],[435,160],[435,30]
			],
			[
				[410,90],[410,30]
			]
		],				
		"place" : [
			{
				"placeName":"トイレ",
				"pos":[140,100],
				"fontSize":60,
			
			},
			{
				"placeName":"11HR",
				"pos":[30,160],
				"fontSize":70,
			
			},
			{
				"placeName":"12HR",
				"pos":[72,160],
				"fontSize":70,
			
			},
			{
				"placeName":"13HR",
				"pos":[112,160],
				"fontSize":70,
			
			},
			{
				"placeName":"14HR",
				"pos":[156,160],
				"fontSize":70,
			
			},
			{
				"placeName":"15HR",
				"pos":[200,160],
				"fontSize":70,
			
			},
			{
				"placeName":"16HR",
				"pos":[245,160],
				"fontSize":70,
			
			},
			{
				"placeName":"17HR",
				"pos":[289,205],
				"fontSize":70,
			
			},
			{
				"placeName":"18HR",
				"pos":[340,205],
				"fontSize":70,
			
			},
			{
				"placeName":"19HR",
				"pos":[393,205],
				"fontSize":70,
			
			},
			{
				"placeName":"10HR",
				"pos":[445,205],
				"fontSize":70,
			
			}
		],
		"Node" : [
			{
				"NodeID":1,
				"pos":[128,110],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":2,
				"pos":[50,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":3,
				"pos":[100,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":4,
				"pos":[128,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":5,
				"pos":[145,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":6,
				"pos":[165,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":7,
				"pos":[210,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":8,
				"pos":[255,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":9,
				"pos":[300,127],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":10,
				"pos":[300,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":11,
				"pos":[350,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":12,
				"pos":[423,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":13,
				"pos":[450,170],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":14,
				"pos":[423,145],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":15,
				"pos":[423,130],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":16,
				"pos":[423,100],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":17,
				"pos":[423,80],
				"radius":3.5,
				"colors":""
			},
			{
				"NodeID":18,
				"pos":[400,130],
				"radius":3.5,
				"colors":""
			}
		],
		"Rect" :[
			{
				"pos" : [],
				"color" : "#00ff00",
			},
			{
				"pos" : [],
				"color" : "#00ff00",
			}
		],


	}
};


let draw = {
	drawPlace : (_context, x, y,data) => {
		let img = new Image();
		img.src = data;
		img.onload = () => {
			console.log(img.width);
			let [width,height] = [img.width, img.height];
			let [dw,dh] = [80,80];
			_context.drawImage(img, 0, 0, width, height, x*magnification - dw/2, (y + 2)*magnification - dh , dw, dh);
		}	
	
	},
	drawLine : (_context, startX, startY, endX, endY, breadth) => {
		startX = startX * magnification - breadth / 2;
		startY = startY * magnification - breadth / 2;
		endX = endX * magnification - breadth / 2;
		endY = endY * magnification - breadth / 2;

		_context.beginPath();
		_context.moveTo(startX,startY);
		_context.lineTo(endX,endY);
		_context.lineTo(endX + breadth,endY + breadth);
		_context.lineTo(startX + breadth,startY + breadth);
		_context.closePath();
		_context.fill();
	},
	drawMap : (_context,json,floor) => {
		let savedStyle = _context.fillStyle;
		_context.fillStyle = BaseFillColor;
		switch(floor) {
			case 1: json = json.map1; break;
			case 2: json = json.map2; break;
			case 3: json = json.map3; break;
			case 4: json = json.map4; break;
			case 5: json = json.map5; break;
		}

		{//draw line for Map
			//To clear the type(I wanna use TypeScript... WebPack isn't easy 4 me)
			let isWidthConf = json.isLineWidthConf ? true : false;
			let width = 2;
			
			for(let i of json.line) {
				let startX,startY;
				
				if(isWidthConf) {
					width = i.lineWidth;
				}

				for(let j of i) {
					//In first loop startX and startY is undefine, this func ignored
					draw.drawLine(_context,startX,startY,j[0],j[1],width);
					[startX,startY] = j;
				}
			}
		}
		{//draw placeName for map
			let savedFont = _context.font;
			_context.fillStyle = BaseFillColor;



			for(let i of json.place) {
				_context.font = String(i.fontSize * magnification) + "%" + " serif";
				_context.fillText(i.placeName, i.pos[0]*magnification,i.pos[1]*magnification);
				
			}
			_context.font = savedFont;
		}
		{//fill react
			for(let i of json.Rect) {
				_context.fillStyle = i.color;
				_context.fillRect(i.pos[0], i.pos[1], i.pos[2], i.pos[3]);
			}
		}
		if(debug_mode)
		{//draw Node for map
			let x,y;					
			let NodeID,radius,color;
			let saveColor = _context.fillStyle;
			for(let i of json.Node) {
				_context.beginPath();
				NodeID = i.NodeID;

				x = i.pos[0] * magnification;
				y = i.pos[1] * magnification;

				radius = i.radius * magnification;
				color = i.colors;
				_context.fillStyle = color;
				_context.arc(x,y,radius,0,360);
				_context.closePath();
				_context.fill();
			}
			_context.fillStyle = saveColor;

		}
		_context.fillStyle = savedStyle;

	},
	connectNode : (_context, json, node1, node2) => {
		if(utils.NodeFloor(node1) !== utils.NodeFloor(node2)) {
			if (debug_mode) {
				console.log("node1 and node2 is not in same floor");
			}
			return;
		}

		let floor = utils.NodeFloor(node1);
		if(debug_mode) {
			console.log("floor",floor);
		}

		switch(floor) {
			case 1: json = json.map1; break;
			case 2: json = json.map2; break;
			case 3: json = json.map3; break;
			case 4: json = json.map4; break;
			case 5: json = json.map5; break;
			default : 
				if(debug_mode) {
					console.log("Error in function connectNode; floor is not on range 1<5");
				}
				return;
		}

		let nodeData = json.Node;
		if (nodeData === undefined) {
			return;
		}
		let pos1, pos2;
		node1 -= utils.FloorVariation(floor);
		node2 -= utils.FloorVariation(floor);


		for(let i of nodeData) {// search the node match nodeID
			if(i.NodeID === node1) {
				pos1 = i.pos;
			}
			if(i.NodeID === node2) {
				pos2 = i.pos;
			}
		}
		if(debug_mode) {
			console.log("ConnectNode");
			console.log(node1, pos1, node2, pos2);
		}

		draw.drawLine(_context,pos1[0],pos1[1],pos2[0],pos2[1],5);
	},
	drawNode : (_context , json , node, color) => {
		let floor = utils.NodeFloor(node);
		
		switch(floor) {
			case 1: json = json.map1; break;
			case 2: json = json.map2; break;
			case 3: json = json.map3; break;
			case 4: json = json.map4; break;
			case 5: json = json.map5; break;
		}

		node -= utils.FloorVariation(floor);

		let x,y;					
		let nodeData = json.Node;
		if(nodeData === undefined) {
			return;
		}
		let radius = 3.5 * magnification;
		let saveColor = _context.fillStyle;
		_context.fillStyle = color;
		let pos;

		for(let i of nodeData) {// search the node match nodeID
			if(i.NodeID === node) {
				pos = i.pos;
			}
		}
		[x,y] = pos;
		x = x * magnification;
		y = y * magnification;

		_context.beginPath();
		_context.arc(x,y,radius,0,360);
		_context.closePath();
		_context.fill();

		_context.fillStyle = saveColor;
		if(debug_mode) {
			console.log("drawNode");
		}
	},
	drawWhere: (node) => {
		if(utils.NodeFloor(node) === FloorNow) {
			draw.drawNode(ctx,map,node,"red");
		}
	},
};
let utils = {
	NodeFloor:(node) => {
		if(0 < node && node <= utils.FloorVariation(2)) {
			return 1;
		}
		else if(utils.FloorVariation(2) < node && node <= utils.FloorVariation(3)){
			return 2
		}
		else if(utils.FloorVariation(3) < node && node <= utils.FloorVariation(4)){
			return 3;
		}
		else if(utils.FloorVariation(4) < node && node <= utils.FloorVariation(5)){
			return 4;
		}
		else if(utils.FloorVariation(5) < node) {
			return 5;
		}

	},
	FloorVariation : (Floor) => {
		switch(Floor) {
			case 1:return 0;
			case 2:return 28;
			case 3:return utils.FloorVariation(2) + 23;
			case 4:return utils.FloorVariation(3) + 18;
			case 5:return utils.FloorVariation(4) + 17;
			default:return -1000;
		}
	},
};


export function drawRoute(route,floor) {
	let json;

	switch (1) {
		case 1: json = map.map1; break;
		case 2: json = map.map2; break;
		case 3: json = map.map3; break;
		case 4: json = map.map4; break;
		case 5: json = map.map5; break;
		default: break;
	}
	if(debug_mode) {
		console.log(route);
	}
	let start_num;
	let goal_num;

	for(let i = 0; i < route.length; ++i) {
		if(utils.NodeFloor(route[i]) === floor) {
			start_num = i;
			break;
		}
	}

	for(let i = 0; i < route.length; ++i) {
		if(utils.NodeFloor(route[i]) === floor) {
			goal_num = i;
			continue;
		}
	}
	for(let i = start_num; i < goal_num; i++) {
		if(debug_mode) {
			console.log("nodes",i, i+1);
		}
		draw.connectNode(ctx, map, route[i], route[i + 1]);
		if(debug_mode) {
			console.log(route[i],route[i+1]);
		}
	}
	if(debug_mode) {
		console.log("route",route);
		console.log(start_num, route[start_num], goal_num, route[goal_num]);
	}
	

	draw.drawNode(ctx, map, route[start_num], "red");
	draw.drawNode(ctx, map, route[goal_num], "red");
}
export function init() {
	board = document.getElementById("canvas");
	ctx = board.getContext("2d");
	board.setAttribute("width",String(canvas_width) + "px");
	board.setAttribute("height",String(canvas_height) + "px");
	ctx.font = "40px serif";
	ctx.fillStyle = "black";
}
export function drawMap(floor) {
	if(ctx === null) {
		if(debug_mode) {
			console.log("Failed to get Context Canvas for Map in canvas.js;");
		}
		return 0;
	}
	if(1 <= floor && floor <= 5) {
		ctx.clearRect(0,0,canvas_width,canvas_height);
		draw.drawMap(ctx, map, floor);
	}
}
export function drawNode(node, color) {
	node = Number(node);
	color = String(color);
	draw.drawNode(ctx, map, 1, color);
}
export function drawConnectNode(node1, node2) {
	draw.connectNode(ctx, map, node1, node2);
}
export function drawPlaceByNode(node, is_goal) {
	let floor = utils.NodeFloor(node);
	let json = map;
	switch(floor) {
		case 1: json = json.map1; break;
		case 2: json = json.map2; break;
		case 3: json = json.map3; break;
		case 4: json = json.map4; break;
		case 5: json = json.map5; break;
		default : 
		return;
	}

	let nodeData = json.Node;
	if (nodeData === undefined) {
		return;
	}
	let pos;
	node -= utils.FloorVariation(floor);

	for(let i of nodeData) {// search the node match nodeID
		if(i.NodeID === node) {
			pos = i.pos;
		}
	}
	let data;
	if(is_goal) {
		data = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMAAAADACAMAAABlApw1AAAABGdBTUEAALGPC/xhBQAAACBjSFJNAAB6JgAAgIQAAPoAAACA6AAAdTAAAOpgAAA6mAAAF3CculE8AAACT1BMVEV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGrxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGrxKGp1dXV1dXV1dXV1dXV1dXXxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGrxKGp1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXV1dXXxKGr///9T9G3HAAAAw3RSTlMAAyRLcpm5x9Tg7frTcUojDk2Pz/nOjkwNLYXK/PvIhCwYc9AWQ7W0QgHGVFIrwL0oC4qHCuJIBY3+iQQgxR1O7mz9Z4J/ApiVqaeWfh/sDOOGUdHNSUCyFxUSBm93Ghx45C7rZmgqpROoyWVp9fZqjMzqFPdrIQflerbE3u+7VdqIk8sbHnvmbd2sPNaXbvF5prexX+HpfKu82A+D8/iLoJznEHTBwzSbYzE5lJoRXiInfS8mT1AJ1bA6dt81oUaqQVN8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";
	}else {
		data = "data:image/png;base64,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";
	}

	draw.drawPlace(ctx, pos[0],pos[1],data);
	console.log(pos[0],pos[1]);
}

/*
I don't have time to rewrite method 
	``` javascript
	switch(floor) {
		case 1: json = json.map1; break;
		case 2: json = json.map2; break;
		case 3: json = json.map3; break;
		case 4: json = json.map4; break;
		case 5: json = json.map5; break;
		default : 
			if(debug_mode) {
				console.log("Error in function connectNode; floor is not on range 1<5");
			}
			return;
	}
	```

	more good using json[] like please
*/