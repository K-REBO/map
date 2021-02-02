if( !window.indexedDB) {
	window.alert("このブラウザは安定版のIndexedDBをサポートしていません。フィードバックください");
}

let db = new Dexie("userData");

db.version(1).stores(
	{
		user:"&key ,HRNO, userName, lastLoginDate, sessionToken"
	}
);
let HRNO,userName;

db.user.get(1).then((record) => {
	console.log(record);
	if(record === undefined) {
		let initDB = 	{
			key: 1,
			HRNO: "debugInt",
			userName: "debugString",
			lastLoginDate: new Date(),
			sessionToken:"ss",
		};

		db.user.put(initDB);

		localStorage.setItem("whereNode","1");
		localStorage.setItem("FloorNow","1");
		localStorage.setItem("destination","1");
	}
	else {
		HRNO = record.HRNO;
		userName = record.userName;
	}
});