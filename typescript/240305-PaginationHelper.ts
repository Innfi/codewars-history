// https://www.codewars.com/kata/515bb423de843ea99400000a

import assert from 'assert';

describe('test', () => {
  class PaginationHelper {
    items: Readonly<any>[];
    perPage: Readonly<number>;
    pageLen: Readonly<number>;

	  constructor(collection: any[], itemsPerPage: number) {
  	  // The constructor takes in an array of items and a integer indicating how many
  	  // items fit within a single page
      this.items = collection;
      this.perPage = itemsPerPage;
      this.pageLen = this.pageCount();
  	}

  	itemCount(): number {
  	  // returns the number of items within the entire collection
      return this.items.length;
  	}

  	pageCount(): number {
  	  // returns the number of pages
      return this.items.length % this.perPage === 0 ? 
        Math.floor(this.items.length / this.perPage) : 
        Math.floor(this.items.length / this.perPage) + 1;
  	}

  	pageItemCount(pageIndex: number): number {
  	  // returns the number of items on the current page. page_index is zero based.
  	  // this method should return -1 for pageIndex values that are out of range
      if (pageIndex < 0 || pageIndex >= this.pageLen) return -1;

      return this.perPage * (pageIndex+1) > this.items.length ?
        this.items.length - this.perPage*pageIndex :
        this.perPage;
  	}

  	pageIndex(itemIndex: number): number {
  	  // determines what page an item is on. Zero based indexes
  	  // this method should return -1 for itemIndex values that are out of range
      if (itemIndex < 0 || itemIndex >= this.items.length) return -1;

      return itemIndex < this.perPage ? 0 : Math.floor(itemIndex / this.perPage);
  	}
  }

  it ('basic cases', () => {
    assert.strictEqual(1, 1);

    assert.strictEqual(Math.floor(6/4)+1, 2);
  });

  it ('test cases', () => {
    const collection = [
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
      22, 23, 24,
    ];
    const helper = new PaginationHelper(collection, 10);

    assert.strictEqual(helper.pageCount(), 3, "pageCount()");
    assert.strictEqual(helper.itemCount(), 24, "itemCount()");

    assert.strictEqual(helper.pageItemCount(1), 10, "pageItemCount(1)");
    assert.strictEqual(helper.pageItemCount(2), 4, "pageItemCount(2)");
    assert.strictEqual(helper.pageItemCount(3), -1, "pageItemCount(3)");

    assert.strictEqual(helper.pageIndex(40), -1, "pageIndex(40)");
    assert.strictEqual(helper.pageIndex(22), 2, "pageIndex(22)");
    assert.strictEqual(helper.pageIndex(3), 0, "pageIndex(3)");
    assert.strictEqual(helper.pageIndex(0), 0, "pageIndex(0)");
    assert.strictEqual(helper.pageIndex(-1), -1, "pageIndex(-1)");
    assert.strictEqual(helper.pageIndex(-23), -1, "pageIndex(-23)");
    assert.strictEqual(helper.pageIndex(-15), -1, "pageIndex(-15)");
  });
});