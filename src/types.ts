export interface Geography{width:number;height:number;bounds:number[];cellDegrees:number;cells:[number,number,number][];polygons:number[][][][];edges:[number,number,number][];provenance:Record<string,string>}
export interface Settings{strategy:string;growth:number;mobility:number;origin:string;source:number}
export interface Snapshot{year:number;population:Float32Array;total:number;occupied:number}
export interface Result{snapshots:Snapshot[];settings:Settings;runtime:number;capacities:Float32Array}
