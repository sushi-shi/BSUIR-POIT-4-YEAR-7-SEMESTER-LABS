----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 04:13:26 PM
-- Design Name: 
-- Module Name: tb_task1_simple_repeat - Behavioral
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

entity tb_task1_simple_repeat is
--  Port ( );
end tb_task1_simple_repeat;

architecture Behavioral of tb_task1_simple_repeat is
    constant i: integer := 2;
	 constant bits: integer := i ** 2;
    constant period: time := 100 ns;
    
    signal Q: STD_LOGIC_VECTOR(0 to bits - 1);
	 
    signal Q_code: STD_LOGIC_VECTOR(0 to 2 * bits - 1);
    signal Q_code_with_err: STD_LOGIC_VECTOR(0 to  2 * bits - 1);
	 
    signal Q_decode: STD_LOGIC_VECTOR(0 to bits - 1);
    signal ERR: STD_LOGIC;
    
begin
    uut_1: entity work.REPEAT_ENCODER 
		generic map(bits) 
		port map(Q, Q_code);
		
    uut_2: entity work.REPEAT_DECODER
		generic map(bits) 
		port map(Q_code_with_err, Q_decode, ERR);
		
    uut_3: entity work.MkError
		generic map(2 * bits - 1) 
		port map (Q_code, Q_code_with_err); 
		
    test_proc: process
    begin
        Q <= "1011";
        wait for period;
        Q <= "1111";
        wait for period;
        Q <= "0000";
        wait for period;
        Q <= "1110";
        wait for period;
    end process;

end Behavioral;
