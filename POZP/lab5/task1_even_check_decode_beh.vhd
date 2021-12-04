----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 03:21:23 PM
-- Design Name: 
-- Module Name: task1_even_check_decode_beh - Behavioral
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

entity EVEN_DECODER is
    generic(i: integer := 4);
    Port (
        Qin: in STD_LOGIC_VECTOR(0 to 2 ** i);
		  
        Qout: out STD_LOGIC_VECTOR(0 to 2 ** i - 1);
        ERR: out STD_LOGIC
    );
end EVEN_DECODER;

architecture Behavioral of EVEN_DECODER is

signal q_t: STD_LOGIC_VECTOR(0 to 2 ** i - 1) := (others => '0');
signal err_t: STD_LOGIC := '0';
	 
begin
    main: process(Qin)
        variable tmp_xor: STD_LOGIC;
    begin
        tmp_xor := '0';
		  
        for i in 0 to 2 ** i - 1 loop
            tmp_xor := tmp_xor xor Qin(i);
        end loop;
		  
        q_t <= Qin(0 to 2 ** i - 1);
        err_t <= Qin(2 ** i) xor tmp_xor;
    end process;
	 
    Qout <= q_t;
    ERR <= err_t;
end Behavioral;
