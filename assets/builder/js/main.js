//-----------------------------------------------------------------------
// MODULE
//

let app = angular.module("app", []);


//-----------------------------------------------------------------------
// DEFAULTS
//

const ASSETS_DIR = "assets/builder/";
const TEXTURE_DIR = "assets/level/";

const DEFAULT_PACKAGE = {
	name: "new_package",
	textures: [],
	levels: [],
};

const DEFAULT_MORPH = {
	position: [0.0, 1.0],
	state: "Metal",
	layer: 0,
};

const DEFAULT_TARGET = {
	position: [0.0, -1.0],
	layer: 0,
}

const DEFAULT_LEVEL = {
	name: "-",
	dimension: [10.0, 10.0],
	available_morphs: {
		Metal: 2,
		Rubber: 2,
		Water: 2,
		Bubble: 2
	},
	morph: DEFAULT_MORPH,
	target: DEFAULT_TARGET,
	objects: [],
};

const DEFAULT_OBJECT = {
	name: "-",
	position: [0.0, 0.0],
	size: [1.0, 1.0],
	rotation: 0.0,
	role: "None",
	texture: -1,
	texture_info: undefined,
	block: undefined,
	accelerator: undefined,
	breakable: undefined,
};

const DEFAULT_TEXTURE_INFO = {
	layer: 5,
	plane: "View",
	animation: 0.0,
}

const DEFAULT_BLOCK = {
	particles: [],
}

const DEFAULT_ACCELERATOR = {
	direction: "Right",
	amplitude: 1.0,
	morph: {
		"Metal": false,
		"Rubber": false,
		"Water": false,
		"Bubble": false
	}
}

const DEFAULT_BREAKABLE = {
	group: 0,
}


//-----------------------------------------------------------------------
// CONTROLLER
//

app.controller("ctrl", ["$scope", "$document", "$timeout", function($scope, $document, $timeout) {
		
	// init
	$scope.zoom = 2.0;	
	$scope.roles = [
		"None",
		"Block",
		"Spikes",
		"Breakable",
		"Grid",
		"Accelerator"
	];
	$scope.acceleratorDirections = [
		"Right",
		"Left",
		"Up",
		"Down"
	]
	$scope.planes = [
		"View",
		"Far",
		"Mid",
		"Near"
	]
	$scope.planeVisibility = {
		"View": true,
		"Far": true,
		"Mid": true,
		"Near": true,
	}
	$scope.tabs = [
		"Package",
		"Level",
		"Object"
	]
	$scope.packageTabs = [
		"Attributes",
		"Levels",
		"Textures"
	]
	$scope.levelTabs = [
		"Attributes",
		"Objects"
	]
	$scope.objectTabs = [
		"Attributes"
	]
	
	$scope.clear = function() {
		$scope.package = angular.copy(DEFAULT_PACKAGE);
		$scope.active_tab = "Package";
		$scope.active_package_tab = "Attributes";
		$scope.active_level_tab = "Attributes";
		$scope.active_object_tab = "Attributes";
		
		$scope.levels = $scope.package.levels;
		$scope.level = null;
		$scope.level_index = null;
		
		$scope.textures = $scope.package.textures;
		$scope.texture_index = null;
		$scope.texture = null;		
		
		$scope.objects = null;
		$scope.object_index = null;
		$scope.object = null;
			
		$scope.drag = null;
	};
	$scope.clear();
	
	// tab handling
	$scope.selectTab = function(tab) {
		$scope.active_tab = tab;
	}
	$scope.selectPackageTab = function(tab) {
		$scope.active_package_tab = tab;
	}
	$scope.selectLevelTab = function(tab) {
		$scope.active_level_tab = tab;
	}
	$scope.selectObjectTab = function(tab) {
		$scope.active_object_tab = tab;
	}
	
	// save and load
	$scope.save = function() {
		let a = $document[0].getElementById('out_file');
		a.href = URL.createObjectURL(new Blob([JSON.stringify($scope.package)], {type: "application/json"}));
		a.click();
	};
	$scope.load = function() {
		$document[0].getElementById('in_file').click();
	};
	$scope.$watch('file', function(files, _) {
		if (files) {
			// clear all
			$scope.clear();
			// read file
			let reader = new FileReader();
			reader.onloadend = function(e) {
				$scope.package = migratePackage(JSON.parse(e.target.result));
				$scope.levels = $scope.package.levels;
				$scope.textures = $scope.package.textures;
				if ($scope.package.levels.length > 0) {
					$scope.selectLevel(0);
				}
				$scope.$apply();
			}
			reader.readAsBinaryString(files[0]);
			
			// reset input
			$document[0].getElementById('in_file').value = "";
			$scope.file = null;
		}		
	});
	
	// zooming
	$scope.zoomIn = function() {
		$scope.unselectObject();
		$scope.zoom *= 1.25;
		$scope.zoom = Math.min($scope.zoom, 8.0);
	}
	
	$scope.zoomOut = function() {
		$scope.unselectObject();
		$scope.zoom *= 0.8;
		$scope.zoom = Math.max($scope.zoom, 0.262144);
	}
	
	// level handling
	$scope.selectLevel = function(index) {
		$scope.unselectObject();
		$scope.level_index = index;
		$scope.level = $scope.levels[$scope.level_index];
		if ($scope.level) {
			$scope.objects = $scope.level.objects;
		}
	};
	$scope.addLevel = function() {
		$scope.levels.push(angular.copy(DEFAULT_LEVEL));
		$scope.selectLevel($scope.levels.length - 1);
	}
	$scope.removeLevel = function() {
		$scope.unselectObject();
		if ($scope.level_index != null) {
			$scope.levels.splice($scope.level_index, 1);
			$scope.level_index = null;
		}
		$scope.selectLevel($scope.level_index);
	}
	$scope.upLevel = function() {
		[$scope.levels[$scope.level_index - 1], $scope.levels[$scope.level_index]] = [$scope.levels[$scope.level_index], $scope.levels[$scope.level_index - 1]];
		$scope.selectLevel($scope.level_index - 1);
	}	
	$scope.downLevel = function() {
		[$scope.levels[$scope.level_index + 1], $scope.levels[$scope.level_index]] = [$scope.levels[$scope.level_index], $scope.levels[$scope.level_index + 1]];
		$scope.selectLevel($scope.level_index + 1);
	}
	
	// texture handling
	$scope.selectTexture = function(index) {
		$scope.texture_index = index;
		$scope.texture = $scope.textures[$scope.texture_index];
	}
	$scope.unselectTexture = function() {
		$scope.texture_index = null;
		$scope.texture = null;
	}
	$scope.addTextures = function() {
		$document[0].getElementById('in_textures').click();
	}
	$scope.addFrames = function() {
		$document[0].getElementById('in_frames').click();
	}
	$scope.$watch('texture_files', function(files, _) {
		if (files && $scope.textures) {
			// add all files
			for (file of files) {
				if (!$scope.textures.includes(file.name)) {
					$scope.textures.push([file.name]);
					$scope.selectTexture($scope.textures.length - 1);
				}
			}
			// reset input
			$document[0].getElementById('in_textures').value = "";
			$scope.texture_files = null;
		}	
	});
	$scope.$watch('frame_files', function(files, _) {
		if (files && $scope.textures && $scope.texture_index != null) {
			// add all frames
			for (file of files) {
				if (!$scope.textures[$scope.texture_index].includes(file.name)) {
					$scope.textures[$scope.texture_index].push(file.name);
				}
			}
			// reset input
			$document[0].getElementById('in_frames').value = "";
			$scope.frame_files = null;
		}	
	});
	$scope.removeTexture = function() {
		if ($scope.texture_index != null) {
			// iterate all objects and remove texture
			$scope.package.levels.forEach(lvl => {
				lvl.objects.forEach(obj => {
					if (obj.texture >= $scope.texture_index) {
						obj.texture--;
					}
				});
			});
			
			$scope.textures.splice($scope.texture_index, 1);
			$scope.texture_index = null;
		}
		$scope.selectTexture($scope.texture_index);
	}
	$scope.textureUrl = function(index) {
		return {"background-image": "url('" + TEXTURE_DIR + $scope.package.name + "/" + $scope.textures[index][0] + "')"};
	}
	
	// object handling
	$scope.addObject = function() {
		if ($scope.objects) {
			$scope.objects.push(angular.copy(DEFAULT_OBJECT));
			$scope.selectObject($scope.objects.length - 1);
		}
	}
	$scope.removeObject = function() {
		if ($scope.object_index != null) {
			$scope.objects.splice($scope.object_index, 1);
			$scope.object_index = null;
		}
		$scope.unselectObject();
	}
	$scope.selectObject = function(index) {
		//$scope.unselectObject();
		$scope.object_index = index;
		$scope.object = $scope.objects[$scope.object_index];
		if ($scope.drag) {
			$scope.drag.forEach(item => { item.disable(); });
		}
		$timeout(function() {
			$scope.drag = subjx('#object-' + index).drag(object_drag_config);
		});
	}
	$scope.unselectObject = function() {
		$scope.object_index = null;
		$scope.object = null;
		if ($scope.drag) {
			$scope.drag.forEach(item => { item.disable(); });
		}
	}
	$scope.selectMorph = function() {
		$scope.unselectObject();
		$timeout(function() {
			$scope.drag = subjx('#morph').drag(morph_drag_config);
		});
	}
	
	$scope.selectTarget = function() {
		$scope.unselectObject();
		$timeout(function() {
			$scope.drag = subjx('#target').drag(target_drag_config);
		});
	}
	
	// drag configs
	const object_drag_config = {
		snap: { x: 1, y: 1, angle: 1},
		onMove({ clientX, clientY, dx, dy, transform }) {
			// fires on moving
			let real = fromStyle($scope.drag[0].el.id);
			$scope.object.size =  real.size;
			$scope.object.position = real.position;
			$scope.object.rotation = real.rotation;
			$scope.$apply();
		},
		onResize({ clientX, clientY, dx, dy, width, height }) {
			// fires on resizing
			let real = fromStyle($scope.drag[0].el.id);
			$scope.object.size =  real.size;
			$scope.object.position = real.position;
			$scope.object.rotation = real.rotation;
			$scope.$apply();
		},
		onRotate({ clientX, clientY, delta, transform }) {
			// fires on rotation
			let real = fromStyle($scope.drag[0].el.id);
			$scope.object.size =  real.size;
			$scope.object.position = real.position;
			$scope.object.rotation = real.rotation;
			$scope.$apply();
		},				
	};
	const morph_drag_config = {
		resizable: false,
		rotatable: false,
		snap: { x: 1, y: 1, angle: 1},
		onMove({ clientX, clientY, dx, dy, transform }) {			
			// fires on moving
			let real = fromStyle($scope.drag[0].el.id);
			$scope.level.morph.position = real.position;
			$scope.$apply();
		},			
	};
	const target_drag_config = {
		resizable: false,
		rotatable: false,
		snap: { x: 1, y: 1, angle: 1},
		onMove({ clientX, clientY, dx, dy, transform }) {
			// fires on moving
			let real = fromStyle($scope.drag[0].el.id);
			$scope.level.target.position = real.position;
			$scope.$apply();
		},			
	};
		
	// object styles
	$scope.morphStyle = function() {
		let style = toStyle($scope.level.morph.position, [1.0, 1.0], 0.0);
		style["z-index"] = planeIndex("View") -  $scope.level.morph.layer;
		style["background-image"] = "url('" + ASSETS_DIR + "img/" + $scope.level.morph.state + ".png')";
		style["visibility"] = $scope.planeVisibility["View"] ? "visible" : "hidden";
		return style;
	}
	
	$scope.targetStyle = function() {
		let style = toStyle($scope.level.target.position, [ 1.5, 1.5], 0.0);
		style["z-index"] = planeIndex("View") -  $scope.level.target.layer;
		style["background-image"] = "url('" + ASSETS_DIR + "img/target.png')";
		style["visibility"] = $scope.planeVisibility["View"] ? "visible" : "hidden";
		return style;
	}
	
	$scope.objectStyle = function(index) {
		let style = toStyle($scope.objects[index].position, $scope.objects[index].size, $scope.objects[index].rotation);
		style["z-index"] = planeIndex("View") - 11;
		style["visibility"] = $scope.planeVisibility["View"] ? "visible" : "hidden";
		if ($scope.objects[index].texture >= 0) {
			style["z-index"] = planeIndex($scope.objects[index].texture_info.plane) - $scope.objects[index].texture_info.layer;
			style["visibility"] = $scope.planeVisibility[$scope.objects[index].texture_info.plane] ? "visible" : "hidden";
			style["background-color"] = "rgba(0,0,0,0)";
			style["background-image"] = "url('" + TEXTURE_DIR + $scope.package.name + "/" + $scope.textures[$scope.objects[index].texture][0] + "')"
		}
		return style;
	}
	
	$scope.objectClass = function(index) {
		return $scope.objects[index].role.toLowerCase();
	}
	
	// keep drag up to date
	$scope.$watch('level.morph', function(n, _) {
		if (n && $scope.drag && $scope.drag[0].storage && $scope.drag[0].el.id.startsWith("morph")) {
			let style = toStyle(n.position, [ 1.0, 1.0], 0.0);
			$scope.drag[0].storage.controls.style.width = style.width;
			$scope.drag[0].storage.controls.style.height = style.height;
			$scope.drag[0].storage.controls.style.transform = style.transform;
		}
	}, true);
	$scope.$watch('level.target', function(n, _) {
		if (n && $scope.drag && $scope.drag[0].storage && $scope.drag[0].el.id.startsWith("target")) {
			let style = toStyle(n.position, [ 1.5, 1.5], 0.0);
			$scope.drag[0].storage.controls.style.width = style.width;
			$scope.drag[0].storage.controls.style.height = style.height;
			$scope.drag[0].storage.controls.style.transform = style.transform;
		}
	}, true);
	$scope.$watch('object', function(n, _) {
		if (n) {
			if ($scope.drag && $scope.drag[0].storage && $scope.drag[0].el.id.startsWith("object")) {
				let style = toStyle(n.position, n.size, n.rotation);
				$scope.drag[0].storage.controls.style.width = style.width;
				$scope.drag[0].storage.controls.style.height = style.height;
				$scope.drag[0].storage.controls.style.transform = style.transform;
			}
			if (n.texture >= 0) {
				if (!n.texture_info) {
					n.texture_info = angular.copy(DEFAULT_TEXTURE_INFO);
				}
				if ($scope.textures[n.texture].length < 2) {
					n.texture_info.animation = 0.0;
				}
			}
			else if (n.texture < 0 && n.texture_info) {
				n.texture_info = undefined;
			}
			if (n.role == "Block" && !n.block) {
				n.block = angular.copy(DEFAULT_BLOCK);
			}
			else if (n.role != "Block" && n.block) {
				n.block = undefined;
			}
			if (n.role == "Accelerator" && !n.accelerator) {
				n.accelerator = angular.copy(DEFAULT_ACCELERATOR);
			}
			else if (n.role != "Accelerator" && n.accelerator) {
				n.accelerator = undefined;
			}
			if (n.role == "Breakable" && !n.breakable) {
				n.breakable = angular.copy(DEFAULT_BREAKABLE);
			}
			else if (n.role != "Breakable" && n.breakable) {
				n.breakable = undefined;
			}
		}
	}, true);

}]);



//-----------------------------------------------------------------------
// DIRECTIVES
//

app.directive('files', function() {
    return {
        require:"ngModel",
        restrict: 'A',
        link: function($scope, el, attrs, ngModel){
            el.bind('change', function(event){
                ngModel.$setViewValue(event.target.files);
                $scope.$apply();
            });
        }
    };
});


//-----------------------------------------------------------------------
// HELPER
//

function migratePackage(pgk) {
	let package = Object.assign({}, DEFAULT_PACKAGE, pgk);
	package["levels"] = package["levels"].map(lvl => {
		let level = Object.assign({}, DEFAULT_LEVEL, lvl);
		level["morph"] = Object.assign({}, DEFAULT_MORPH, level["morph"]);
		level["target"] = Object.assign({}, DEFAULT_TARGET, level["target"]);
		level["objects"] = level["objects"].map(obj => {
			let object = Object.assign({}, DEFAULT_OBJECT, obj);
			if (object["role"] == "Block" && !object["block"]) {
				object["block"] = angular.copy(DEFAULT_BLOCK);
			}
			if (object["role"] == "Breakable" && !object["breakable"]) {
				object["breakable"] = angular.copy(DEFAULT_BREAKABLE);
			}
			return object;
		});
		return level
	});
	// need to migrate textures?
	if (package["textures"].length > 0 && !Array.isArray(package["textures"][0])) {
		package["levels"].forEach(lvl => lvl["objects"].map(obj => {
			obj["texture"] = package["textures"].indexOf(obj["texture"]);
			if (obj["texture"] >= 0) {
				obj["texture_info"] = angular.copy(DEFAULT_TEXTURE_INFO);
				obj["texture_info"]["layer"] = obj["layer"];
				obj["texture_info"]["plane"] = obj["texture_plane"];
			}
			obj["layer"] = undefined;
			obj["texture_plane"] = undefined;
		}));
		package["textures"] = package["textures"].map(tex => [tex]);
	}
	return package;
}

function toStyle(pos, size, rotate) {
	let width = toPx(size[0] * 2.0);
	let height = toPx(size[1] * 2.0);
	let pos_x = toPx(pos[0] - size[0]);
	let pos_y = -toPx(pos[1] + size[1]);
	let matrix = [Math.cos(rotate), -Math.sin(rotate), Math.sin(rotate), Math.cos(rotate), pos_x, pos_y];
	return {
		"width": width + "px",
		"height": height + "px",
		"transform": "matrix(" + matrix[0] + "," + matrix[1] + "," + matrix[2] + "," + matrix[3] + "," + matrix[4] + "," + matrix[5] + ")"
	}
}

function fromStyle(id) {
	let el = document.getElementById(id);
	let size_x = toEm(el.offsetWidth / 2.0);
	let size_y = toEm(el.offsetHeight / 2.0);
	let matrix = el.style.transform.split(/matrix\(|,|\)/).filter(v => v !== '');
	let pos_x = toEm(matrix[4]) + size_x;
	let pos_y = -toEm(matrix[5]) - size_y;
	let rot = -Math.atan2(matrix[1], matrix[0]);
	return { size: [size_x, size_y ], position: [ pos_x, pos_y ], rotation: rot };
}

function toEm(px) {
	return px / (document.getElementById("sizer").offsetHeight / 100);
}

function toPx(em) {
	return em * (document.getElementById("sizer").offsetHeight / 100);
}

function planeIndex(plane) {
	switch (plane) {
		case "Near": return 400;
		case "View": return 300;
		case "Mid":	return 200;
		case "Far": return 100;
	}
}