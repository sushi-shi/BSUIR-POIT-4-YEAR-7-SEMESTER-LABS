----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 04:37:59 PM
-- Design Name: 
-- Module Name: task1_hamming_code_beh - Behavioral
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

entity HAMMING_ENCODE is
    generic (
        bits: integer;
        control_bits: integer
    );
    Port (
        Qin: in STD_LOGIC_VECTOR(0 to bits - 1);
        Qout: out STD_LOGIC_VECTOR(0 to bits + control_bits - 1)
    );
end HAMMING_ENCODE;

architecture Behavioral of HAMMING_ENCODE is
begin
    main: process(Qin)
        variable q_t: STD_LOGIC_VECTOR(0 to bits + control_bits - 1);
		          
        variable tmp_index: integer;
        variable tmp_xor: STD_LOGIC;
		  
        variable step: integer;
    begin
        q_t := (others => '1');
        for i in 0 to control_bits - 1 loop
            q_t(2 ** i - 1) := '0';
        end loop;
       
        tmp_index := 0;
        for i in 0 to bits + control_bits - 1 loop
            if q_t(i) = '1' then
                q_t(i) := Qin(tmp_index);
                tmp_index := tmp_index + 1;
            end if;
        end loop;
        
        for i in 0 to control_bits - 1 loop
            step := 2 ** i;
            tmp_index := step - 1;
            tmp_xor := '0';
            while tmp_index <= bits + control_bits - 1 loop
                for j in 0 to step - 1 loop
                    if tmp_index <= bits + control_bits - 1 then
                        tmp_xor := tmp_xor xor q_t(tmp_index);
                        tmp_index := tmp_index + 1;
                    end if;
                end loop;
                tmp_index := tmp_index + step;
            end loop;
            q_t(step - 1) := tmp_xor;
        end loop;
        
        Qout <= q_t;
    end process;
    
end Behavioral;
