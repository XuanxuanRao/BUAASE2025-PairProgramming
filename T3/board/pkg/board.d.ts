/* tslint:disable */
/* eslint-disable */
export function process_turn(n: number, snakes: Int32Array, actions: Int32Array, foods: Int32Array, seed: bigint): OutputBuffer;
export function initial_foods(n: number, snakes: Int32Array, seed: bigint, required_count: number): Int32Array;
export class OutputBuffer {
  private constructor();
  free(): void;
  status(): number;
  snakes(): number;
  foods(): number;
}
