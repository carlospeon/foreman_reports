import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  createSolidTable,
  getPaginationRowModel,
} from '@tanstack/solid-table';
import { createSignal } from 'solid-js';

function Filter(props) {
  // const firstValue = table
  //   .getPreFilteredRowModel()
  //   .flatRows[0]?.getValue(column.id)

  const column = props.column;
  const columnFilterValue = column.getFilterValue();
  return (
    <div class="tfilter">
      {/* <datalist id={column.id + 'list'}>
        {sortedUniqueValues.slice(0, 10000).map((value) => (
          <option value={value} key={value} />
        ))}
      </datalist> */}
      <input class="tfilter"
        type="text"
        value={(columnFilterValue ?? '')}
        onChange={e => column.setFilterValue(e.target.value)}
        placeholder="filter"
        // list={column.id + 'list'}
      />
    </div>
  )
}

export function ReportTable(props) {
  const [sorting, setSorting] = createSignal([]);
  const [columnFilters, setColumnFilters] = createSignal([]);
  const [globalFilter, setGlobalFilter] = createSignal();

  var options = props.options;
  options.state = {
    get sorting()       { return sorting(); },
    get columnFilters() { return columnFilters(); },
    get globalFilter()  { return globalFilter(); },
  };
  options.onSortingChange = setSorting;
  options.onColumnFiltersChange = setColumnFilters;
  options.onGlobalFilterChange = setGlobalFilter;
  options.getCoreRowModel = getCoreRowModel();
  options.getSortedRowModel = getSortedRowModel();
  options.getFilteredRowModel = getFilteredRowModel();
  options.getPaginationRowModel = getPaginationRowModel();

  options.initialState = { pagination: { pageSize: 25 } };
  

  const table = createSolidTable(options);
  return (
    <>
      <div>
        <input
          value={globalFilter() ?? ''}
          onchange={e => setGlobalFilter(e.target.value)}
          placeholder="Search "
        />
      </div>
      <table class="report">
        <thead>
          <For each={table.getHeaderGroups()}>
            {headerGroup => (
              <tr>
                <For each={headerGroup.headers}>
                  {header => (
                    <th class={header.column.columnDef.accessorKey}>
                      <div class={header.column.columnDef.class +
                              (header.column.getCanSort()?' cursor-pointer':'')}
                        onClick={header.column.getToggleSortingHandler()}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                        {{
                            asc: ' 🔼',
                            desc: ' 🔽',
                          }[header.column.getIsSorted()] ?? ' ⥯ '}
                        </div>
                        {header.column.getCanFilter() ? (
                            <Filter column={header.column} table={table} />
                        ) : null}
                    </th>
                  )}
                </For>
              </tr>
            )}
          </For>
        </thead>
        <tbody>
          <For each={table.getRowModel().rows}>
            {row => (
              <tr>
                <For each={row.getVisibleCells()}>
                  {cell => (
                    <td class={cell.column.columnDef.class}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </td>
                  )}
                </For>
              </tr>
            )}
          </For>
        </tbody>
        <tfoot>
          <For each={table.getFooterGroups()}>
            {footerGroup => (
              <tr>
                <For each={footerGroup.headers}>
                  {header => (
                    <th class={header.column.columnDef.class}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                          header.column.columnDef.footer,
                          header.getContext()
                        )}
                    </th>
                  )}
                </For>
              </tr>
            )}
          </For>
        </tfoot>
      </table>
      <div className="pagination">
        <button
          onClick={() => table.setPageIndex(0)}
          disabled={!table.getCanPreviousPage()}
        >
          {'<<'}
        </button>
        <button
          onClick={() => table.previousPage()}
          disabled={!table.getCanPreviousPage()}
        >
          {'<'}
        </button>
        <button
          onClick={() => table.nextPage()}
          disabled={!table.getCanNextPage()}
        >
          {'>'}
        </button>
        <button
          onClick={() => table.setPageIndex(table.getPageCount() - 1)}
          disabled={!table.getCanNextPage()}
        >
          {'>>'}
        </button>
        <span>
          Page 
          <strong>
            {table.getState().pagination.pageIndex + 1} of{' '}
            {table.getPageCount()}
          </strong>
        </span>
        <span>
          | Go to page:
          <input
            type="number"
            defaultValue={table.getState().pagination.pageIndex + 1}
            onChange={e => {
              const page = e.target.value ? Number(e.target.value) - 1 : 0
              table.setPageIndex(page)
            }}
          />
        </span>
        <select
          value={table.getState().pagination.pageSize}
          onChange={e => {
            table.setPageSize(Number(e.target.value))
          }}
        >
          {[25, 50, 100, 200, 500].map(pageSize => (
            <option key={pageSize} value={pageSize}>
              Show {pageSize}
            </option>
          ))}
        </select>
      </div>
    </>
  )
}

export function LegendTable(props) {
  const [sorting, setSorting] = createSignal([]);

  var options = props.options;
  options.state = {
    get sorting()       { return sorting(); },
  };
  options.onSortingChange = setSorting;
  options.getCoreRowModel = getCoreRowModel();
  options.getSortedRowModel = getSortedRowModel();

  const table = createSolidTable(options);
  const showHeader = (typeof props.showHeader === 'undefined')?false:props.showHeader;
  return (
    <div>
      <table class="legend">
        <Show when={showHeader}>
        <thead>
          <For each={table.getHeaderGroups()}>
            {headerGroup => (
              <tr>
                <For each={headerGroup.headers}>
                  {header => (
                    <th>
                      <div class={header.column.columnDef.class +
                              (header.column.getCanSort()?' cursor-pointer':'')}
                        onClick={header.column.getToggleSortingHandler()}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                        {{
                            asc: ' 🔼',
                            desc: ' 🔽',
                          }[header.column.getIsSorted()] ?? ' ⥯ '}
                        </div>
                    </th>
                  )}
                </For>
              </tr>
            )}
          </For>
        </thead>
        </Show>
        <tbody>
          <For each={table.getRowModel().rows}>
            {row => (
              <tr>
                <For each={row.getVisibleCells()}>
                  {cell => (
                    <td class={cell.column.columnDef.class}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </td>
                  )}
                </For>
              </tr>
            )}
          </For>
        </tbody>
        <tfoot>
          <For each={table.getFooterGroups()}>
            {footerGroup => (
              <tr>
                <For each={footerGroup.headers}>
                  {header => (
                    <th class={header.column.columnDef.class}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                          header.column.columnDef.footer,
                          header.getContext()
                        )}
                    </th>
                  )}
                </For>
              </tr>
            )}
          </For>
        </tfoot>
      </table>
    </div>
  
  )
}
