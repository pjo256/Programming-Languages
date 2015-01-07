with Ada.Text_IO; use Ada.Text_IO;
with Ada.Integer_Text_IO; use Ada.Integer_Text_IO;
package body QuickSort is

    procedure quickSort(numbers: in out Arr; low, high: Integer) is
        pivotIndex : Integer;
        mid : Integer;
        
        function medianIndex(median : in out Integer; i, j, k : in Integer) return Integer is
            l : Integer := numbers(i);
            m : Integer := numbers(k);
            h : Integer := numbers(j);
            minVal : Integer;
            
            function min(l, h : in Integer) return Integer is
            begin
                if l <= h then
                    return l;
                else 
                    return h;
                end if;
            end min;
        begin
            if m < h and then m > l then    
                median := m;
                return k;
            elsif l < h and then l > m then
                median := l;
                return i;
            elsif h < l and then h > m then
                median := h;
                return j;
            else 
                minVal := min(m, min(l, h));
                median := minVal;
                if minVal = l then
                    return i;
                elsif minVal = m then
                    return k;
                else
                   return j; 
                end if;
            end if;
        end medianIndex;
         
        procedure swap(a, b : in out Integer) is
            temp : constant Integer := a;
        begin
            a := b;
            b := temp;
        end swap;
    
        function partition return Integer is
            i : Integer := low + 1;
            j : Integer := high;
            med : Integer := 0;
            pivotIndex : Integer;
        begin
            pivotIndex := medianIndex(med, low, mid, high);
            swap(numbers(pivotIndex), numbers(low));
            while i < j loop
                while i < j and then numbers(i) < med loop
                    i := i + 1;
                end loop;     
                while i < j and then numbers(j) > med loop
                    j := j - 1;
                end loop;       
                if i < j then
                    swap(numbers(i), numbers(j));
                    i := i + 1;
                    j := j - 1;
                end if;
            end loop;
            if numbers(i) < med then
                pivotIndex := i;
            else 
                pivotIndex := i - 1;
            end if;
            swap(numbers(low), numbers(pivotIndex));
            return pivotIndex;         
        end partition; 
    
        procedure sortHelper is
            task leftSort;       
            task rightSort;
    
            task body leftSort is
            begin
                quickSort(numbers, low, pivotIndex - 1);
            end leftSort;
        
            task body rightSort is
            begin
                quickSort(numbers, pivotIndex + 1, high);
            end rightSort;
        
        begin
            null;
        end sortHelper;             
    begin
        if high - low <= 0 then
            return;
        else
            mid := (low + high) / 2;
            pivotIndex := partition;
            sortHelper;
        end if;
    end quickSort;
end QuickSort;
    