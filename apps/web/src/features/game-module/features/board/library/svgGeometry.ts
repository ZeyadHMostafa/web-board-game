export const SvgGeometry = {
  /**
   * Returns the exact top-left corner of a 100x100 vector square.
   */
  toTileTopLeft(row: number, col: number) {
    return {
      x: col * 100,
      y: (7 - row) * 100
    };
  },

  /**
   * Returns the exact center point of a 100x100 vector square.
   */
  toTileCenter(row: number, col: number) {
    return {
      x: (col * 100) + 50,
      y: ((7 - row) * 100) + 50
    };
  }
};