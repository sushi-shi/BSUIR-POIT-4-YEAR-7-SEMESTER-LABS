----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/29/2021 06:12:31 PM
-- Design Name: 
-- Module Name: task4_control_schema_beh - Behavioral
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

entity task4_control_schema_beh is
    generic (
        bitness_address: integer := 2;
        bitness_data: integer := 4;
        control_bits_amount: integer := 3
    );
    Port (
        CLK: in STD_LOGIC;
        RW: in STD_LOGIC;
        A: in STD_LOGIC_VECTOR((bitness_address - 1) downto 0);
        D_in: in STD_LOGIC_VECTOR((bitness_data + control_bits_amount - 1) downto 0);
        D_out: out STD_LOGIC_VECTOR((bitness_data + control_bits_amount - 1) downto 0)
    );
end task4_control_schema_beh;

architecture Behavioral of task4_control_schema_beh is
    signal encoded_D: STD_LOGIC_VECTOR((bitness_data + control_bits_amount - 1) downto 0);
    signal decoded_D: STD_LOGIC_VECTOR((bitness_data - 1) downto 0);
    signal D_from_ram: STD_LOGIC_VECTOR((bitness_data - 1) downto 0);
    --signal D_from_ram: STD_LOGIC_VECTOR((bitness_data - 1) downto 0);
begin
    encoder: entity work.task1_hamming_code_beh generic map (bitness => bitness_data, control_bits_amount => control_bits_amount)
                                                port map (D_from_ram, encoded_D);
                                                
    decoder: entity work.task1_hamming_decode_beh generic map (bitness => bitness_data, control_bits_amount => control_bits_amount)
                                                  port map (D_in, decoded_D);
                                                  
    ram: entity work.task4_ram_beh generic map (bitness_address => bitness_address, bitness_data => bitness_data)
                                   port map (CLK, RW, A, decoded_D, D_from_ram);
    D_out <= encoded_D;
end Behavioral;
