----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 04:37:59 PM
-- Design Name: 
-- Module Name: task1_hamming_decode_beh - Behavioral
-- Project Name: 
-- Target Devices: 
-- Tool Versions: 
-- Description: 
-- 
-- Dependencies: 
-- 
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
-- 
----------------------------------------------------------------------------------


library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity HAMMING_DECODE is
    generic (
        bits: integer;
        control_bits: integer
    );
    Port (
	     Qin: in STD_LOGIC_VECTOR(0 to bits + control_bits - 1);
        Qout: out STD_LOGIC_VECTOR(0 to bits - 1)
    );
end HAMMING_DECODE;

architecture Behavioral of HAMMING_DECODE is
    signal q_t_res: STD_LOGIC_VECTOR(0 to bits - 1);
begin
    main: process(Qin)
	 	  variable q_t: STD_LOGIC_VECTOR(0 to bits - 1);
	     variable qin_t: STD_LOGIC_VECTOR(0 to bits + control_bits - 1);
		  variable control_bits_positions: STD_LOGIC_VECTOR(0 to bits + control_bits - 1);

        variable tmp_index: integer := 0;
        variable step: integer := 0;        
        variable tmp_xor: STD_LOGIC := '0';        
    begin
        qin_t := (others => '1');
        for i in 0 to control_bits - 1 loop
            qin_t(2 ** i - 1) := '0';
        end loop;
       
        tmp_index := 0;
        for i in 0 to bits + control_bits - 1 loop
            if qin_t(i) = '1' then
                qin_t(i) := Qin(i);
            end if;
        end loop;
        
		  for i in 0 to control_bits - 1 loop
            step := 2 ** i;
            tmp_index := step - 1;
            tmp_xor := '0';
            while tmp_index <= bits + control_bits - 1 loop
                for j in 0 to step - 1 loop
                    if tmp_index <= bits + control_bits - 1 then
                        tmp_xor := tmp_xor xor qin_t(tmp_index);
                        tmp_index := tmp_index + 1;
                    end if;
                end loop;
                tmp_index := tmp_index + step;
            end loop;
            qin_t(step - 1) := tmp_xor;
        end loop;
        
        tmp_index := -1;
        for i in 0 to control_bits - 1 loop
				if Qin(2 ** i - 1) /= qin_t(2 ** i - 1) then
					tmp_index := tmp_index + i + 1;
				end if;         
        end loop;

        if tmp_index /= -1 and tmp_index  <= bits + control_bits - 1 then
            qin_t(tmp_index) := not qin_t(tmp_index);
        end if;
		  
		  control_bits_positions := (others => '0');
        for i in 0 to control_bits - 1 loop
            control_bits_positions(2 ** i - 1) := '1';
        end loop;
        
        tmp_index := 0;
        for i in 0 to bits + control_bits - 1 loop
            if (control_bits_positions(i) /= '1') then
                q_t(tmp_index) := qin_t(i);
                tmp_index := tmp_index + 1;
            end if;
        end loop;
		  
		  q_t_res <= q_t;
    end process;
	 
    Qout <= q_t_res;
	 
end Behavioral;
