----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/27/2021 04:37:59 PM
-- Design Name: 
-- Module Name: tb_task1_hamming - Behavioral
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

entity tb_task1_hamming is
--  Port ( );
end tb_task1_hamming;

architecture Behavioral of tb_task1_hamming is
    constant bits: integer := 16;
	 constant control_bits: integer := 5;
    constant period: time := 100 ns;
    
    signal Q: STD_LOGIC_VECTOR(0 to bits - 1) := "0100010000111101";
    signal Q_code: STD_LOGIC_VECTOR(0 to bits + control_bits - 1);
    signal Q_code_with_err: STD_LOGIC_VECTOR(0 to bits + control_bits - 1);
    signal Q_decode: STD_LOGIC_VECTOR(0 to bits - 1);
    signal ERR: STD_LOGIC_VECTOR(0 to bits - 1);
    
begin    
    uut_1: entity work.HAMMING_ENCODE 
		generic map(bits, control_bits) 
      port map(Q, Q_code);
                                                                         
    uut_make_error: entity work.MkError 
		generic map(bits + control_bits - 1) 
      port map(Q_code, Q_code_with_err);
		
    uut_2: entity work.HAMMING_DECODE
		generic map(bits, control_bits) 
      port map(Q_code, Q_decode);
    
    ERR <= Q xor Q_decode;
    
    test_proc: process
    begin
        Q <= "0100010000111101";
        wait for period;
    end process;

end Behavioral;
